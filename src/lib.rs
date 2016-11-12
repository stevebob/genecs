extern crate tomson;
extern crate handlebars;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::mem;

use tomson::Toml;
use handlebars::Handlebars;
use rustc_serialize::json::{self, Json};


const TEMPLATE: &'static str = r#"// Automatically generated. Do not edit.
use std::collections::{BTreeMap, BTreeSet};
use std::cell::{Cell, UnsafeCell};
use std::slice;
use std::usize;

{{#each imports}}
use {{ this }};
{{/each}}

pub type EntityId = u64;

pub type EntityMap<T> = BTreeMap<EntityId, T>;
pub type EntitySet = BTreeSet<EntityId>;

pub const NUM_COMPONENTS: usize = {{num_components}};

const WORD_SIZE: usize = {{word_size}};
const WORD_BITS: usize = {{word_bits}};

const COMPONENT_SET_NUM_WORDS: usize = {{component_set_num_words}};

pub type ComponentType = usize;

pub mod component_type {
    use std::usize;

{{#each component}}
    pub const {{id_uppercase}}: usize = {{index}};
{{/each}}
    pub const INVALID_COMPONENT: usize = usize::MAX;
}

pub struct ComponentTypeSet {
    bitfields: [usize; COMPONENT_SET_NUM_WORDS],
}

pub struct ComponentTypeSetIter {
    bitfields: [usize; COMPONENT_SET_NUM_WORDS],
    index: usize,
}

impl ComponentTypeSetIter {
    fn new(bitfields: [usize; COMPONENT_SET_NUM_WORDS]) -> Self {
        ComponentTypeSetIter {
            bitfields: bitfields,
            index: 0,
        }
    }
}

impl Iterator for ComponentTypeSetIter {
    type Item = ComponentType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < COMPONENT_SET_NUM_WORDS && self.bitfields[self.index] == 0 {
            self.index += 1;
        }
        if self.index == COMPONENT_SET_NUM_WORDS {
            return None;
        }

        let trailing = self.bitfields[self.index].trailing_zeros() as usize;
        self.bitfields[self.index] &= !(1 << trailing);
        Some(self.index * WORD_BITS + trailing)
    }
}

impl ComponentTypeSet {
    pub fn new() -> Self {
        ComponentTypeSet {
            bitfields: [0; COMPONENT_SET_NUM_WORDS],
        }
    }

    pub fn is_empty(&self) -> bool {
        for b in &self.bitfields {
            if *b != 0 {
                return false;
            }
        }

        true
    }

    pub fn clear(&mut self) {
        for b in &mut self.bitfields {
            *b = 0;
        }
    }

    pub fn iter(&self) -> ComponentTypeSetIter {
        ComponentTypeSetIter::new(self.bitfields)
    }

{{#each component}}
    pub fn contains_{{id}}(&self) -> bool {
        self.bitfields[{{set_index}}] & (1 << {{set_bit}}) != 0
    }

    pub fn insert_{{id}}(&mut self) {
        self.bitfields[{{set_index}}] |= 1 << {{set_bit}};
    }

    pub fn remove_{{id}}(&mut self) {
        self.bitfields[{{set_index}}] &= !(1 << {{set_bit}});
    }
{{/each}}
}

struct ComponentDirtyFlags {
    insert: bool,
    remove: bool,
}

impl ComponentDirtyFlags {
    fn new() -> Self {
        ComponentDirtyFlags {
            insert: false,
            remove: false,
        }
    }

    fn clean(&mut self) {
        self.insert = false;
        self.remove = false;
    }
}

struct DirtyComponentTracker {
{{#each queried_components}}
    {{ @key }}: ComponentDirtyFlags,
{{/each}}
}

impl DirtyComponentTracker {
    fn new() -> Self {
        DirtyComponentTracker {
{{#each queried_components}}
            {{ @key }}: ComponentDirtyFlags::new(),
{{/each}}
        }
    }

{{#each query}}
    fn should_populate_{{id}}(&self) -> bool {
        (true {{#each components}}&& self.{{id}}.insert {{/each}}) ||
        (false {{#each components}}|| self.{{id}}.remove {{/each}})
    }
{{/each}}
}

pub struct EcsTable {
{{#each component}}
    {{id}}: {{#if type}} EntityMap<{{type}}> {{else}} EntitySet {{/if}},
{{/each}}
}

impl EcsTable {
    pub fn new() -> Self {
        EcsTable {
{{#each component}}
            {{id}}: {{#if type}} EntityMap::new() {{else}} EntitySet::new() {{/if}},
{{/each}}
        }
    }

{{#each component}}
    {{#if type}}
    pub fn insert_{{id}}(&mut self, entity: EntityId, value: {{type}}) {
        self.{{id}}.insert(entity, value);
    }

    pub fn contains_{{id}}(&self, entity: EntityId) -> bool {
        self.{{id}}.contains_key(&entity)
    }

    pub fn {{id}}(&self, entity: EntityId) -> Option<&{{type}}> {
        self.{{id}}.get(&entity)
    }

    pub fn {{id}}_mut(&mut self, entity: EntityId) -> Option<&mut {{type}}> {
        self.{{id}}.get_mut(&entity)
    }
    {{else}}
    pub fn insert_{{id}}(&mut self, entity: EntityId) {
        self.{{id}}.insert(entity);
    }

    pub fn contains_{{id}}(&self, entity: EntityId) -> bool {
        self.{{id}}.contains(&entity)
    }
    {{/if}}

    pub fn remove_{{id}}(&mut self, entity: EntityId) {
        self.{{id}}.remove(&entity);
    }

    pub fn count_{{id}}(&self) -> usize {
        self.{{id}}.len()
    }
{{/each}}

    pub fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        match component_type {
{{#each component}}
            component_type::{{id_uppercase}} => self.remove_{{id}}(entity),
{{/each}}
            _ => panic!("Invalid component type: {}", component_type),
        }
    }

    pub fn remove_components(&mut self, entity: EntityId, component_type_set: ComponentTypeSet) {
        for component_type in component_type_set.iter() {
            self.remove_component(entity, component_type);
        }
    }

    pub fn push_component_entity_ids(&self, component_type: ComponentType, ids: &mut Vec<EntityId>) {
        match component_type {
{{#each component}}
    {{#if type}}
            component_type::{{id_uppercase}} => {
                for id in self.{{id}}.keys() {
                    ids.push(*id);
                }
            },
    {{else}}
            component_type::{{id_uppercase}} => {
                for id in self.{{id}}.iter() {
                    ids.push(*id);
                }
            },
    {{/if}}
{{/each}}
            _ => panic!("Invalid component type: {}", component_type),
        }
    }
}

pub struct EcsCtx {
    table: EcsTable,
    tracker: EntityMap<ComponentTypeSet>,
    next_id: Cell<EntityId>,
    query_ctx: UnsafeCell<QueryCtx>,
}

impl EcsCtx {
    pub fn new() -> Self {
        EcsCtx {
            table: EcsTable::new(),
            tracker: EntityMap::new(),
            next_id: Cell::new(0),
            query_ctx: UnsafeCell::new(QueryCtx::new()),
        }
    }

    fn query_ctx_mut(&self) -> &mut QueryCtx {
        unsafe {
            &mut *self.query_ctx.get()
        }
    }

{{#each component}}
    {{#if type}}
    pub fn insert_{{id}}(&mut self, entity: EntityId, value: {{type}}) {
        self.table.insert_{{id}}(entity, value);
        self.tracker.entry(entity).or_insert_with(ComponentTypeSet::new).insert_{{id}}();
        {{#if queried}}
        self.set_dirty_insert_{{id}}();
        {{/if}}
    }

    pub fn contains_{{id}}(&self, entity: EntityId) -> bool {
        self.table.contains_{{id}}(entity)
    }

    pub fn {{id}}(&self, entity: EntityId) -> Option<&{{type}}> {
        self.table.{{id}}(entity)
    }

    pub fn {{id}}_mut(&mut self, entity: EntityId) -> Option<&mut {{type}}> {
        self.table.{{id}}_mut(entity)
    }
    {{else}}
    pub fn insert_{{id}}(&mut self, entity: EntityId) {
        self.table.insert_{{id}}(entity);
        self.tracker.entry(entity).or_insert_with(ComponentTypeSet::new).insert_{{id}}();
        {{#if queried}}
        self.set_dirty_insert_{{id}}();
        {{/if}}
    }

    pub fn contains_{{id}}(&self, entity: EntityId) -> bool {
        self.table.contains_{{id}}(entity)
    }
    {{/if}}

    pub fn remove_{{id}}(&mut self, entity: EntityId) {
        self.table.remove_{{id}}(entity);
        let empty = self.tracker.get_mut(&entity).map(|set| {
            set.remove_{{id}}();
            set.is_empty()
        });
        if let Some(true) = empty {
            self.tracker.remove(&entity);
        }
        {{#if queried}}
        self.set_dirty_remove_{{id}}();
        {{/if}}
    }

    {{#if queried}}
    fn set_dirty_insert_{{id}}(&self) {
        self.query_ctx_mut().dirty.{{id}}.insert = true;
    }
    fn set_dirty_remove_{{id}}(&self) {
        self.query_ctx_mut().dirty.{{id}}.remove = true;
    }
    {{/if}}
{{/each}}

    pub fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        match component_type {
{{#each component}}
            component_type::{{id_uppercase}} => self.remove_{{id}}(entity),
{{/each}}
            _ => panic!("Invalid component type: {}", component_type),
        }
    }

    pub fn remove_components(&mut self, entity: EntityId, component_type_set: ComponentTypeSet) {
        for component_type in component_type_set.iter() {
            self.remove_component(entity, component_type);
        }
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        if let Some(set) = self.tracker.remove(&entity) {
            self.table.remove_components(entity, set);
        }
    }

    pub fn entity(&self, id: EntityId) -> Option<EntityRef> {
        if id < self.next_id.get() {
            Some(EntityRef::new(id, self))
        } else {
            None
        }
    }

    pub fn entity_mut(&mut self, id: EntityId) -> Option<EntityRefMut> {
        if id < self.next_id.get() {
            Some(EntityRefMut::new(id, self))
        } else {
            None
        }
    }

    pub fn alloc_entity_id(&self) -> EntityId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

    pub fn alloc_entity(&self) -> EntityRef {
        EntityRef::new(self.alloc_entity_id(), self)
    }

    pub fn alloc_entity_mut(&mut self) -> EntityRefMut {
        EntityRefMut::new(self.alloc_entity_id(), self)
    }

{{#each query}}
    pub fn {{id}}(&self) -> {{prefix}}Iter {
        let query_ctx = self.query_ctx_mut();
        if query_ctx.dirty.should_populate_{{id}}() {

            // identify the component with the least number of entities
            let mut _max = usize::MAX;
            let mut component_type = component_type::INVALID_COMPONENT;

    {{#each components}}
            let count = self.table.count_{{id}}();
            if count < _max {
                _max = count;
                component_type = component_type::{{id_uppercase}};
            }
    {{/each}}

            // collect the ids of the component
            query_ctx.tmp_entity_ids.clear();
            self.table.push_component_entity_ids(component_type, &mut query_ctx.tmp_entity_ids);

            // populate the results
            query_ctx.{{id}}.results.clear();

            for id in query_ctx.tmp_entity_ids.iter() {
    {{#each components}}
        {{#if type}}
                let {{id}} = if let Some(component) = self.table.{{id}}(*id) {
                    component as *const {{type}}
                } else {
                    continue;
                };
        {{/if}}
    {{/each}}

                let result = {{prefix}}InnerResult {
                    id: *id,
    {{#each components}}
        {{#if type}}
                    {{id}}: {{id}},
        {{/if}}
    {{/each}}
                };

                query_ctx.{{id}}.results.push(result);
            }

    {{#each components}}
            query_ctx.dirty.{{id}}.clean();
    {{/each}}
        }

        query_ctx.{{id}}.iter()
    }
{{/each}}
}

#[derive(Clone, Copy)]
pub struct EntityRef<'a> {
    id: EntityId,
    ctx: &'a EcsCtx,
}

impl<'a> EntityRef<'a> {
    fn new(id: EntityId, ctx: &'a EcsCtx) -> Self {
        EntityRef {
            id: id,
            ctx: ctx,
        }
    }

    pub fn id(self) -> EntityId {
        self.id
    }

    pub fn is_empty(self) -> bool {
        if let Some(set) = self.ctx.tracker.get(&self.id) {
            set.is_empty()
        } else {
            true
        }
    }

{{#each component}}
    pub fn contains_{{id}}(self) -> bool {
        self.ctx.contains_{{id}}(self.id)
    }
    {{#if type}}
    pub fn {{id}}(self) -> Option<&'a {{type}}> {
        self.ctx.{{id}}(self.id)
    }
    {{/if}}
{{/each}}
}

pub struct EntityRefMut<'a> {
    id: EntityId,
    ctx: &'a mut EcsCtx,
}

impl<'a> EntityRefMut<'a> {
    fn new(id: EntityId, ctx: &'a mut EcsCtx) -> Self {
        EntityRefMut {
            id: id,
            ctx: ctx,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        if let Some(set) = self.ctx.tracker.get(&self.id) {
            set.is_empty()
        } else {
            true
        }
    }

    pub fn destroy(self) {
        self.ctx.remove_entity(self.id);
    }
{{#each component}}
    pub fn contains_{{id}}(&self) -> bool {
        self.ctx.contains_{{id}}(self.id)
    }
    pub fn remove_{{id}}(&mut self) {
        self.ctx.remove_{{id}}(self.id)
    }
    {{#if type}}
    pub fn {{id}}(&self) -> Option<&{{type}}> {
        self.ctx.{{id}}(self.id)
    }
    pub fn {{id}}_mut(&mut self) -> Option<&mut {{type}}> {
        self.ctx.{{id}}_mut(self.id)
    }
    pub fn insert_{{id}}(&mut self, value: {{type}}) {
        self.ctx.insert_{{id}}(self.id, value);
    }
    {{else}}
    pub fn insert_{{id}}(&mut self) {
        self.ctx.insert_{{id}}(self.id);
    }
    {{/if}}
{{/each}}
}

{{#each query}}
pub struct {{prefix}}Result<'a> {
    id: EntityId,
    {{#each components}}
        {{#if type}}
    {{id}}: &'a {{type}},
        {{/if}}
    {{/each}}
}

pub struct {{prefix}}Iter<'a> {
    slice_iter: slice::Iter<'a, {{prefix}}InnerResult>,
}

impl<'a> Iterator for {{prefix}}Iter<'a> {
    type Item = {{prefix}}Result<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.slice_iter.next().map(|inner| {
            inner.to_outer_result()
        })
    }
}

struct {{prefix}}InnerResult {
    id: EntityId,
    {{#each components}}
        {{#if type}}
    {{id}}: *const {{type}},
        {{/if}}
    {{/each}}
}

impl {{prefix}}InnerResult {
    fn to_outer_result(&self) -> {{prefix}}Result {
        unsafe {
            {{prefix}}Result {
                id: self.id,
{{#each components}}
    {{#if type}}
                {{id}}: &(*self.{{id}}),
    {{/if}}
{{/each}}
            }
        }
    }
}

struct {{prefix}}QueryCtx {
    results: Vec<{{prefix}}InnerResult>,
}

impl {{prefix}}QueryCtx {
    fn new() -> Self {
        {{prefix}}QueryCtx {
            results: Vec::new(),
        }
    }

    fn iter(&self) -> {{prefix}}Iter {
        {{prefix}}Iter {
            slice_iter: self.results.iter(),
        }
    }
}
{{/each}}

struct QueryCtx {
    tmp_entity_ids: Vec<EntityId>,
    dirty: DirtyComponentTracker,
{{#each query}}
    {{id}}: {{prefix}}QueryCtx,
{{/each}}
}

impl QueryCtx {
    fn new() -> Self {
        QueryCtx {
            tmp_entity_ids: Vec::new(),
            dirty: DirtyComponentTracker::new(),
{{#each query}}
            {{id}}: {{prefix}}QueryCtx::new(),
{{/each}}
        }
    }
}
"#;

fn generate_code(mut toml: String) -> String {
    // turn the toml string into json for compatibility with handlebars
    let mut json = Toml::as_json(&mut toml).unwrap();

    let mut component_clones = HashMap::new();

    let num_components = json.search("component").unwrap().as_object().unwrap().len();
    json.as_object_mut().unwrap().insert("num_components".to_string(), Json::U64(num_components as u64));

    let word_size = mem::size_of::<usize>();
    json.as_object_mut().unwrap().insert("word_size".to_string(), Json::U64(word_size as u64));

    let word_bits = word_size * 8;
    json.as_object_mut().unwrap().insert("word_bits".to_string(), Json::U64(word_bits as u64));

    let component_set_num_words = (num_components - 1) / word_bits + 1;
    json.as_object_mut().unwrap().insert("component_set_num_words".to_string(), Json::U64(component_set_num_words as u64));

    let mut queried_components = json::Object::new();
    for query in json.as_object().unwrap().get("query").unwrap().as_object().unwrap().values() {
        let query_obj = query.as_object().unwrap();
        let components_json = query_obj.get("components").unwrap();
        let components_arr = components_json.as_array().unwrap();
        for component in components_arr {
            let component_str = component.as_string().unwrap();
            queried_components.insert(component_str.to_string(), Json::Boolean(true));
        }
    }

    let mut index = 0;
    for (id, component) in json.as_object_mut().unwrap().get_mut("component").unwrap().as_object_mut().unwrap().iter_mut() {
        let component_obj = component.as_object_mut().unwrap();
        component_obj.insert("index".to_string(), Json::U64(index as u64));
        component_obj.insert("set_index".to_string(), Json::U64((index / word_bits) as u64));
        component_obj.insert("set_bit".to_string(), Json::U64((index % word_bits) as u64));
        component_obj.insert("id".to_string(), Json::String(id.to_string()));
        component_obj.insert("id_uppercase".to_string(), Json::String(id.to_uppercase()));

        if queried_components.contains_key(id) {
            component_obj.insert("queried".to_string(), Json::Boolean(true));
        }

        component_clones.insert(id.to_string(), component_obj.clone());

        index += 1;
    }

    for (id, query) in json.as_object_mut().unwrap().get_mut("query").unwrap().as_object_mut().unwrap().iter_mut() {
        let query_obj = query.as_object_mut().unwrap();
        query_obj.insert("id".to_string(), Json::String(id.to_string()));
        let components_json = query_obj.remove("components").unwrap();
        let components_arr = components_json.as_array().unwrap();
        let mut components_obj = json::Object::new();
        for component in components_arr {
            let component_str = component.as_string().unwrap();
            let clone = component_clones.get(component_str).unwrap().clone();
            components_obj.insert(component_str.to_string(), Json::Object(clone));
        }
        query_obj.insert("components".to_string(), Json::Object(components_obj));
    }

    json.as_object_mut().unwrap().insert("queried_components".to_string(), Json::Object(queried_components));

    let mut handlebars = Handlebars::new();

    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars.template_render(TEMPLATE, &json).unwrap()
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();

    string
}

pub fn generate_ecs<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) {

    let string = read_file_to_string(in_path);

    let output_string = generate_code(string);

    let mut outfile = File::create(out_path).unwrap();
    write!(outfile, "{}", output_string).unwrap();
}
