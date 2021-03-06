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
use rustc_serialize::json::Json;


const TEMPLATE: &'static str = r#"// Automatically generated. Do not edit.
#![allow(unused_imports)]

use std::collections::{BTreeMap, btree_map, BTreeSet, btree_set, HashMap, hash_map, HashSet, hash_set};
use std::cell::{UnsafeCell, RefCell, Ref, RefMut};
use std::slice;
use std::usize;
use std::vec;
use std::mem;

{{#each imports}}
use {{ this }};
{{/each}}

pub type EntityId = u64;

#[derive(Serialize, Deserialize)]
pub struct EntityMap<T> {
    inner: BTreeMap<EntityId, T>,
}

impl<T> EntityMap<T> {
    pub fn new() -> Self {
        EntityMap {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, value: T) {
        self.inner.insert(entity, value);
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.inner.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.inner.get_mut(&entity)
    }

    pub fn contains_key(&self, entity: EntityId) -> bool {
        self.inner.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.inner.remove(&entity)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn entry(&mut self, entity: EntityId) -> btree_map::Entry<EntityId, T> {
        self.inner.entry(entity)
    }

    pub fn iter(&self) -> EntityMapIter<T> {
        EntityMapIter::new(self.inner.iter())
    }

    pub fn keys(&self) -> EntityMapKeys<T> {
        EntityMapKeys::new(self.inner.keys())
    }
}

impl<T: Copy> EntityMap<T> {
    pub fn copy_iter(&self) -> EntityMapCopyIter<T> {
        EntityMapCopyIter::new(self.inner.iter())
    }
}

pub struct EntityMapKeys<'a, T: 'a> {
    keys: btree_map::Keys<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityMapKeys<'a, T> {
    fn new(keys: btree_map::Keys<'a, EntityId, T>) -> Self {
        EntityMapKeys {
            keys: keys,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityMapIter<'a, T: 'a> {
    iter: btree_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityMapIter<'a, T> {
    fn new(iter: btree_map::Iter<'a, EntityId, T>) -> Self {
        EntityMapIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

pub struct EntityMapCopyIter<'a, T: 'a + Copy> {
    iter: btree_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a + Copy> EntityMapCopyIter<'a, T> {
    fn new(iter: btree_map::Iter<'a, EntityId, T>) -> Self {
        EntityMapCopyIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a + Copy> Iterator for EntityMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}

#[derive(Serialize, Deserialize)]
pub struct EntitySet {
    inner: BTreeSet<EntityId>,
}

impl EntitySet {
    pub fn new() -> Self {
        EntitySet {
            inner: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId) {
        self.inner.insert(entity);
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        self.inner.remove(&entity)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.inner.contains(&entity)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn iter(&self) -> EntitySetIter {
        EntitySetIter::new(self.inner.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub struct EntitySetIter<'a> {
    iter: btree_set::Iter<'a, EntityId>,
}

impl<'a> EntitySetIter<'a> {
    fn new(iter: btree_set::Iter<'a, EntityId>) -> Self {
        EntitySetIter {
            iter: iter,
        }
    }
}

impl<'a> Iterator for EntitySetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityHashMap<T> {
    inner: HashMap<EntityId, T>,
}

impl<T> EntityHashMap<T> {
    pub fn new() -> Self {
        EntityHashMap {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, value: T) {
        self.inner.insert(entity, value);
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.inner.get(&entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.inner.get_mut(&entity)
    }

    pub fn contains_key(&self, entity: EntityId) -> bool {
        self.inner.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.inner.remove(&entity)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn entry(&mut self, entity: EntityId) -> hash_map::Entry<EntityId, T> {
        self.inner.entry(entity)
    }

    pub fn iter(&self) -> EntityHashMapIter<T> {
        EntityHashMapIter::new(self.inner.iter())
    }

    pub fn keys(&self) -> EntityHashMapKeys<T> {
        EntityHashMapKeys::new(self.inner.keys())
    }

    pub fn drain(&mut self) -> hash_map::Drain<EntityId, T> {
        self.inner.drain()
    }
}

impl<T: Copy> EntityHashMap<T> {
    pub fn copy_iter(&self) -> EntityHashMapCopyIter<T> {
        EntityHashMapCopyIter::new(self.inner.iter())
    }
}

pub struct EntityHashMapKeys<'a, T: 'a> {
    keys: hash_map::Keys<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityHashMapKeys<'a, T> {
    fn new(keys: hash_map::Keys<'a, EntityId, T>) -> Self {
        EntityHashMapKeys {
            keys: keys,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityHashMapKeys<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next().map(|id_ref| *id_ref)
    }
}

pub struct EntityHashMapCopyIter<'a, T: 'a + Copy> {
    iter: hash_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a + Copy> EntityHashMapCopyIter<'a, T> {
    fn new(iter: hash_map::Iter<'a, EntityId, T>) -> Self {
        EntityHashMapCopyIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a + Copy> Iterator for EntityHashMapCopyIter<'a, T> {
    type Item = (EntityId, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, *v))
    }
}

pub struct EntityHashMapIter<'a, T: 'a> {
    iter: hash_map::Iter<'a, EntityId, T>,
}

impl<'a, T: 'a> EntityHashMapIter<'a, T> {
    fn new(iter: hash_map::Iter<'a, EntityId, T>) -> Self {
        EntityHashMapIter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a> Iterator for EntityHashMapIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id_ref, v)| (*id_ref, v))
    }
}

pub struct EntityHashSet {
    inner: HashSet<EntityId>,
}

impl EntityHashSet {
    pub fn new() -> Self {
        EntityHashSet {
            inner: HashSet::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId) {
        self.inner.insert(entity);
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        self.inner.remove(&entity)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.inner.contains(&entity)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn drain(&mut self) -> hash_set::Drain<EntityId> {
        self.inner.drain()
    }

    pub fn iter(&self) -> EntityHashSetIter {
        EntityHashSetIter::new(self.inner.iter())
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub struct EntityHashSetIter<'a> {
    iter: hash_set::Iter<'a, EntityId>,
}

impl<'a> EntityHashSetIter<'a> {
    fn new(iter: hash_set::Iter<'a, EntityId>) -> Self {
        EntityHashSetIter {
            iter: iter,
        }
    }
}

impl<'a> Iterator for EntityHashSetIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id_ref| *id_ref)
    }
}

pub const NUM_COMPONENTS: usize = {{num_components}};

const WORD_SIZE: usize = {{word_size}};
const WORD_BITS: usize = {{word_bits}};

const COMPONENT_TYPE_SET_NUM_WORDS: usize = {{component_set_num_words}};

pub type ComponentType = usize;

pub mod component_type {
    use std::usize;

{{#each component}}
    pub const {{id_uppercase}}: usize = {{index}};
{{/each}}
    pub const INVALID_COMPONENT: usize = usize::MAX;
}

#[derive(Serialize, Deserialize)]
pub struct ComponentTypeSet {
    bitfields: [usize; COMPONENT_TYPE_SET_NUM_WORDS],
}

pub struct ComponentTypeSetIter {
    bitfields: [usize; COMPONENT_TYPE_SET_NUM_WORDS],
    index: usize,
}

impl ComponentTypeSetIter {
    fn new(bitfields: [usize; COMPONENT_TYPE_SET_NUM_WORDS]) -> Self {
        ComponentTypeSetIter {
            bitfields: bitfields,
            index: 0,
        }
    }
}

impl Iterator for ComponentTypeSetIter {
    type Item = ComponentType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < COMPONENT_TYPE_SET_NUM_WORDS && self.bitfields[self.index] == 0 {
            self.index += 1;
        }
        if self.index == COMPONENT_TYPE_SET_NUM_WORDS {
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
            bitfields: [0; COMPONENT_TYPE_SET_NUM_WORDS],
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

pub struct EcsCtx {
{{#each component}}
    {{#if type}}
        {{#if container}}
    {{id}}: EntityMap<{{container}}<{{type}}>>,
        {{else}}
    {{id}}: EntityMap<{{type}}>,
        {{/if}}
    {{else}}
    {{id}}: EntitySet,
    {{/if}}
{{/each}}
    tracker: EntityMap<ComponentTypeSet>,
}

impl EcsCtx {
    pub fn new() -> Self {
        EcsCtx {
{{#each component}}
            {{id}}: {{#if type}} EntityMap::new() {{else}} EntitySet::new() {{/if}},
{{/each}}
            tracker: EntityMap::new(),
        }
    }

{{#each component}}
    {{#if type}}

        {{#if container}}
    pub fn bare_insert_{{id}}(&mut self, entity: EntityId, value: {{container}}<{{type}}>) {
        self.{{id}}.insert(entity, value);
        self.tracker.entry(entity).or_insert_with(ComponentTypeSet::new).insert_{{id}}();
    }

    pub fn bare_remove_{{id}}(&mut self, entity: EntityId) -> Option<{{container}}<{{type}}>> {
        let ret = self.{{id}}.remove(entity);
        let empty = self.tracker.get_mut(entity).map(|set| {
            set.remove_{{id}}();
            set.is_empty()
        });
        if let Some(true) = empty {
            self.tracker.remove(entity);
        }

        ret
    }
        {{/if}}

    pub fn insert_{{id}}(&mut self, entity: EntityId, value: {{type}}) {
        {{#if container}}
        self.bare_insert_{{id}}(entity, {{container}}::new(value));
        {{else}}
        self.{{id}}.insert(entity, value);
        {{/if}}
        self.tracker.entry(entity).or_insert_with(ComponentTypeSet::new).insert_{{id}}();
    }

    pub fn contains_{{id}}(&self, entity: EntityId) -> bool {
        self.{{id}}.contains_key(entity)
    }

        {{#if copy}}
    pub fn {{id}}(&self, entity: EntityId) -> Option<{{type}}> {
        self.{{id}}.get(entity).map(|r| *r)
    }
    pub fn {{id}}_ref(&self, entity: EntityId) -> Option<&{{type}}> {
        self.{{id}}.get(entity)
    }
        {{else}}
            {{#if container}}
    pub fn {{id}}(&self, entity: EntityId) -> Option<&{{container}}<{{type}}>> {
        self.{{id}}.get(entity)
    }
                {{#if RefCell}}
    pub fn {{id}}_borrow(&self, entity: EntityId) -> Option<Ref<{{type}}>> {
        self.{{id}}.get(entity).map(|e| e.borrow())
    }
    pub fn {{id}}_borrow_mut(&self, entity: EntityId) -> Option<RefMut<{{type}}>> {
        self.{{id}}.get(entity).map(|e| e.borrow_mut())
    }
                {{/if}}
                {{#if UnsafeCell}}
    pub fn {{id}}_unsafe_get_mut(&self, entity: EntityId) -> Option<*mut {{type}}> {
        self.{{id}}.get(entity).map(|e| e.get())
    }
    pub fn {{id}}_unsafe_get(&self, entity: EntityId) -> Option<*const {{type}}> {
        self.{{id}}.get(entity).map(|e| e.get() as *const {{type}})
    }
                {{/if}}
            {{else}}
    pub fn {{id}}(&self, entity: EntityId) -> Option<&{{type}}> {
        self.{{id}}.get(entity)
    }
            {{/if}}
        {{/if}}

        {{#if container}}
    pub fn {{id}}_mut(&mut self, entity: EntityId) -> Option<&mut {{container}}<{{type}}>> {
        self.{{id}}.get_mut(entity)
    }
        {{else}}
    pub fn {{id}}_mut(&mut self, entity: EntityId) -> Option<&mut {{type}}> {
        self.{{id}}.get_mut(entity)
    }
        {{/if}}
    {{else}}
    pub fn insert_{{id}}(&mut self, entity: EntityId) {
        self.{{id}}.insert(entity);
        self.tracker.entry(entity).or_insert_with(ComponentTypeSet::new).insert_{{id}}();
    }

    pub fn contains_{{id}}(&self, entity: EntityId) -> bool {
        self.{{id}}.contains(entity)
    }
    {{/if}}

    {{#if container}}
    pub fn remove_{{id}}(&mut self, entity: EntityId) -> Option<{{type}}> {
        self.bare_remove_{{id}}(entity).map(|c| c.into_inner())
    }
    {{else}}
    pub fn remove_{{id}}(&mut self, entity: EntityId)
        {{#if type}}
        -> Option<{{type}}>
        {{else}}
        -> bool
        {{/if}}
    {
        let ret = self.{{id}}.remove(entity);
        let empty = self.tracker.get_mut(entity).map(|set| {
            set.remove_{{id}}();
            set.is_empty()
        });
        if let Some(true) = empty {
            self.tracker.remove(entity);
        }

        ret
    }
    {{/if}}

    pub fn remove_{{id}}_into(&mut self, entity: EntityId, action: &mut EcsAction) {
    {{#if type}}
        self.remove_{{id}}(entity).map(|component| {
            action.insert_{{id}}(entity, component);
        });
    {{else}}
        if self.remove_{{id}}(entity) {
            action.insert_{{id}}(entity);
        }
    {{/if}}
    }

    pub fn move_{{id}}(&mut self, src: EntityId, dst: EntityId) {
    {{#if type}}
        {{#if container}}
        self.bare_remove_{{id}}(src).map(|x| {
            self.bare_insert_{{id}}(dst, x);
        });
        {{else}}
        self.remove_{{id}}(src).map(|x| {
            self.insert_{{id}}(dst, x);
        });
        {{/if}}
    {{else}}
        if self.remove_{{id}}(src) {
            self.insert_{{id}}(dst);
        }
    {{/if}}
    }

    pub fn swap_{{id}}(&mut self, a: EntityId, b: EntityId) {
    {{#if type}}
        {{#if container}}

        let a_component = self.bare_remove_{{id}}(a);
        let b_component = self.bare_remove_{{id}}(b);

        if let Some(a_component) = a_component {
            self.bare_insert_{{id}}(b, a_component);
        }

        if let Some(b_component) = b_component {
            self.bare_insert_{{id}}(a, b_component);
        }

        {{else}}

        let a_component = self.remove_{{id}}(a);
        let b_component = self.remove_{{id}}(b);

        if let Some(a_component) = a_component {
            self.insert_{{id}}(b, a_component);
        }

        if let Some(b_component) = b_component {
            self.insert_{{id}}(a, b_component);
        }

        {{/if}}
    {{else}}
        let a_contains = self.contains_{{id}}(a);

        if self.contains_{{id}}(b) {
            self.insert_{{id}}(a);
        } else {
            self.remove_{{id}}(a);
        }

        if a_contains {
            self.insert_{{id}}(b);
        } else {
            self.remove_{{id}}(b);
        }
    {{/if}}
    }
{{/each}}

    pub fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        match component_type {
{{#each component}}
            component_type::{{id_uppercase}} => { self.remove_{{id}}(entity); }
{{/each}}
            _ => { panic!("Invalid component type: {}", component_type); }
        }
    }

    pub fn remove_components(&mut self, entity: EntityId, component_type_set: ComponentTypeSet) {
        for component_type in component_type_set.iter() {
            self.remove_component(entity, component_type);
        }
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        if let Some(set) = self.tracker.remove(entity) {
            for component_type in set.iter() {
                self.remove_component(entity, component_type);
            }
        }
    }

    pub fn entity(&self, id: EntityId) -> EntityRef {
        EntityRef::new(id, self)
    }

    pub fn entity_mut(&mut self, id: EntityId) -> EntityRefMut {
        EntityRefMut::new(id, self)
    }

    pub fn post_action_entity<'a>(&'a self, id: EntityId, action: &'a EcsAction) -> PostActionEntityRef<'a> {
        PostActionEntityRef::new(id, self, action)
    }


    pub fn commit(&mut self, action: &mut EcsAction) {

{{#each component}}

        if action.changed_components.contains_{{id}}() {

    {{#if type}}

            for (id, component) in action.{{id}}.insertions.drain() {
                self.insert_{{id}}(id, component);
            }

    {{else}}

            for id in action.{{id}}.insertions.drain() {
                self.insert_{{id}}(id);
            }

    {{/if}}

            for id in action.{{id}}.removals.drain() {
                self.remove_{{id}}(id);
            }

            for (a, b) in action.{{id}}.swaps.apply.drain(..) {
                self.swap_{{id}}(a, b);
            }
            action.{{id}}.swaps.lookup.clear();

            for mv in action.{{id}}.moves.apply.drain(..) {
                self.move_{{id}}(mv.source, mv.destination);
            }
            action.{{id}}.moves.lookup_to.clear();
            action.{{id}}.moves.lookup_from.clear();
        }
{{/each}}

        action.changed_components.clear();
        action.properties.clear();
    }

    pub fn commit_into(&mut self, from: &mut EcsAction, to: &mut EcsAction) {
{{#each component}}

        if from.changed_components.contains_{{id}}() {

    {{#if type}}

            for (id, component) in from.{{id}}.insertions.drain() {
                self.insert_{{id}}(id, component);
            }

    {{else}}

            for id in from.{{id}}.insertions.drain() {
                self.insert_{{id}}(id);
            }

    {{/if}}

            for id in from.{{id}}.removals.drain() {
                self.remove_{{id}}_into(id, to);
            }

            for (a, b) in from.{{id}}.swaps.apply.drain(..) {
                self.swap_{{id}}(a, b);
            }
            from.{{id}}.swaps.lookup.clear();

            for mv in from.{{id}}.moves.apply.drain(..) {
                self.move_{{id}}(mv.source, mv.destination);
            }
            from.{{id}}.moves.lookup_to.clear();
            from.{{id}}.moves.lookup_from.clear();
        }
{{/each}}

        from.changed_components.clear();
        from.properties.clear();
    }

    pub fn entity_iter<I: Iterator<Item=EntityId>>(&self, iter: I) -> EntityRefIter<I> {
        EntityRefIter::new(self, iter)
    }

{{#each component}}
    {{#if type}}

    pub fn {{id}}_id_iter(&self) ->
        {{#if container}}
            {{#if RefCell}}
        EntityMapKeys<RefCell<{{type}}>>
            {{/if}}
            {{#if UnsafeCell}}
        EntityMapKeys<UnsafeCell<{{type}}>>
            {{/if}}
        {{else}}
        EntityMapKeys<{{type}}>
        {{/if}}

    {
        self.{{id}}.keys()
    }

    pub fn {{id}}_iter(&self) ->
        {{#if container}}
            {{#if RefCell}}
        EntityMapIter<RefCell<{{type}}>>
            {{/if}}
            {{#if UnsafeCell}}
        EntityMapIter<UnsafeCell<{{type}}>>
            {{/if}}
        {{else}}
            {{#if copy}}
        EntityMapCopyIter<{{type}}>
            {{else}}
        EntityMapIter<{{type}}>
            {{/if}}
        {{/if}}

    {
        {{#if copy}}
            self.{{id}}.copy_iter()
        {{else}}
            self.{{id}}.iter()
        {{/if}}
    }

    {{else}}

    pub fn {{id}}_id_iter(&self) -> EntitySetIter {
        self.{{id}}.iter()
    }

    {{/if}}

{{/each}}

    pub fn clear(&mut self) {
{{#each component}}
        self.{{id}}.clear();
{{/each}}
        self.tracker.clear();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableEcsCtx {
{{#each component}}
    {{#if type}}
    {{id}}: EntityMap<{{type}}>,
    {{else}}
    {{id}}: EntitySet,
    {{/if}}
{{/each}}
    tracker: EntityMap<ComponentTypeSet>,
}

impl From<SerializableEcsCtx> for EcsCtx {
    fn from(ecs: SerializableEcsCtx) -> Self {
        let SerializableEcsCtx {
{{#each component}}
    {{#if container}}
            mut {{id}},
    {{else}}
            {{id}},
    {{/if}}
{{/each}}
            tracker,
        } = ecs;

        EcsCtx {
{{#each component}}
    {{#if type}}
        {{#if container}}
            {{#if RefCell}}
            {{id}}: {
                let mut map = EntityMap::new();
                let keys: Vec<EntityId> = {{id}}.keys().collect();
                for key in keys {
                    if let Some(value) = {{id}}.remove(key) {
                        map.insert(key, RefCell::new(value));
                    }
                }

                map
            },
            {{/if}}
            {{#if UnsafeCell}}
            {{id}}: {
                let mut map = EntityMap::new();
                let keys: Vec<EntityId> = {{id}}.keys().collect();
                for key in keys {
                    if let Some(value) = {{id}}.remove(key) {
                        map.insert(key, UnsafeCell::new(value));
                    }
                }

                map
            },
            {{/if}}
        {{else}}
            {{id}}: {{id}},
        {{/if}}
    {{else}}
            {{id}}: {{id}},
    {{/if}}
{{/each}}
            tracker: tracker,
        }
    }
}

impl From<EcsCtx> for SerializableEcsCtx {
    fn from(ecs: EcsCtx) -> Self {
        let EcsCtx {
{{#each component}}
    {{#if container}}
            mut {{id}},
    {{else}}
            {{id}},
    {{/if}}
{{/each}}
            tracker,
        } = ecs;

        SerializableEcsCtx {
{{#each component}}
    {{#if type}}
        {{#if container}}
            {{#if RefCell}}
            {{id}}: {
                let mut map = EntityMap::new();
                let keys: Vec<EntityId> = {{id}}.keys().collect();
                for key in keys {
                    if let Some(cell) = {{id}}.remove(key) {
                        map.insert(key, cell.into_inner());
                    }
                }

                map
            },
            {{/if}}
            {{#if UnsafeCell}}
            {{id}}: {
                let mut map = EntityMap::new();
                let keys: Vec<EntityId> = {{id}}.keys().collect();
                for key in keys {
                    if let Some(cell) = {{id}}.remove(key) {
                        let ptr = cell.get();
                        assert!(!ptr.is_null(), "Attempt to serialize null UnsafeCell");
                        map.insert(key, unsafe { cell.into_inner() });
                    }
                }

                map
            },
            {{/if}}
        {{else}}
            {{id}}: {{id}},
        {{/if}}
    {{else}}
            {{id}}: {{id}},
    {{/if}}
{{/each}}
            tracker: tracker,
        }
    }
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
        if let Some(set) = self.ctx.tracker.get(self.id) {
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
        {{#if copy}}
    pub fn {{id}}(self) -> Option<{{type}}> {
        self.ctx.{{id}}(self.id)
    }
    pub fn {{id}}_ref(self) -> Option<&'a {{type}}> {
        self.ctx.{{id}}_ref(self.id)
    }
        {{else}}
            {{#if container}}
    pub fn {{id}}(self) -> Option<&'a {{container}}<{{type}}>> {
        self.ctx.{{id}}(self.id)
    }
                {{#if RefCell}}
    pub fn {{id}}_borrow(self) -> Option<Ref<'a, {{type}}>> {
        self.ctx.{{id}}_borrow(self.id)
    }
    pub fn {{id}}_borrow_mut(self) -> Option<RefMut<'a, {{type}}>> {
        self.ctx.{{id}}_borrow_mut(self.id)
    }
                {{/if}}
                {{#if UnsafeCell}}
    pub fn {{id}}_unsafe_get_mut(self) -> Option<*mut {{type}}> {
        self.ctx.{{id}}_unsafe_get_mut(self.id)
    }
    pub fn {{id}}_unsafe_get(self) -> Option<*const {{type}}> {
        self.ctx.{{id}}_unsafe_get(self.id)
    }
                {{/if}}
            {{else}}
    pub fn {{id}}(self) -> Option<&'a {{type}}> {
        self.ctx.{{id}}(self.id)
    }
            {{/if}}
        {{/if}}
    {{/if}}
{{/each}}
}

pub trait EntityPopulate {
{{#each component}}
    {{#if type}}
    fn insert_{{id}}(&mut self, value: {{type}});
    {{else}}
    fn insert_{{id}}(&mut self);
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
        if let Some(set) = self.ctx.tracker.get(self.id) {
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

    {{#if type}}
        {{#if container}}
    pub fn bare_remove_{{id}}(&mut self) -> Option<{{container}}<{{type}}>> {
        self.ctx.bare_remove_{{id}}(self.id)
    }
        {{/if}}
    {{/if}}

    pub fn remove_{{id}}(&mut self)
    {{#if type}}
        -> Option<{{type}}>
    {{else}}
        -> bool
    {{/if}}
    {
        self.ctx.remove_{{id}}(self.id)
    }
    {{#if type}}
        {{#if copy}}
    pub fn {{id}}(&self) -> Option<{{type}}> {
        self.ctx.{{id}}(self.id)
    }
    pub fn {{id}}_ref(&self) -> Option<&{{type}}> {
        self.ctx.{{id}}_ref(self.id)
    }
        {{else}}
            {{#if container}}
    pub fn {{id}}(&self) -> Option<&{{container}}<{{type}}>> {
        self.ctx.{{id}}(self.id)
    }
                {{#if RefCell}}
    pub fn {{id}}_borrow(&self) -> Option<Ref<{{type}}>> {
        self.ctx.{{id}}_borrow(self.id)
    }
    pub fn {{id}}_borrow_mut(&self) -> Option<RefMut<{{type}}>> {
        self.ctx.{{id}}_borrow_mut(self.id)
    }
                {{/if}}
                {{#if UnsafeCell}}
    pub fn {{id}}_unsafe_get_mut(&self) -> Option<*mut {{type}}> {
        self.ctx.{{id}}_unsafe_get_mut(self.id)
    }
    pub fn {{id}}_unsafe_get(&self) -> Option<*const {{type}}> {
        self.ctx.{{id}}_unsafe_get(self.id)
    }
                {{/if}}
            {{else}}
    pub fn {{id}}(&self) -> Option<&{{type}}> {
        self.ctx.{{id}}(self.id)
    }
            {{/if}}
        {{/if}}
        {{#if container}}
    pub fn {{id}}_mut(&mut self) -> Option<&mut {{container}}<{{type}}>> {
        self.ctx.{{id}}_mut(self.id)
    }
        {{else}}
    pub fn {{id}}_mut(&mut self) -> Option<&mut {{type}}> {
        self.ctx.{{id}}_mut(self.id)
    }
        {{/if}}
    {{/if}}
{{/each}}
}

impl<'a> EntityPopulate for EntityRefMut<'a> {
{{#each component}}
    {{#if type}}
    fn insert_{{id}}(&mut self, value: {{type}}) {
        self.ctx.insert_{{id}}(self.id, value);
    }
    {{else}}
    fn insert_{{id}}(&mut self) {
        self.ctx.insert_{{id}}(self.id);
    }
    {{/if}}
{{/each}}
}

impl<'a> EntityPopulate for ActionEntityRefMut<'a> {
{{#each component}}
    {{#if type}}
    fn insert_{{id}}(&mut self, value: {{type}}) {
        self.action.insert_{{id}}(self.id, value);
    }
    {{else}}
    fn insert_{{id}}(&mut self) {
        self.action.insert_{{id}}(self.id);
    }
    {{/if}}
{{/each}}
}

struct FlagMoveIter<'a> {
    components: &'a EntitySet,
    iter: btree_map::Iter<'a, EntityId, EntityId>,
}

impl<'a> FlagMoveIter<'a> {
    fn new(components: &'a EntitySet, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        FlagMoveIter {
            components: components,
            iter: iter,
        }
    }
}

struct FlagMovePositiveIter<'a>(FlagMoveIter<'a>);

impl<'a> FlagMovePositiveIter<'a> {
    fn new(components: &'a EntitySet, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        FlagMovePositiveIter(FlagMoveIter::new(components, iter))
    }
}

impl<'a> Iterator for FlagMovePositiveIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((dest, src)) = self.0.iter.next() {
            // for each flag being moved into dest
            if self.0.components.contains(*src) {
                // if the flag was set in the source
                if !self.0.components.contains(*dest) {
                    // if the flag wasn't set in the destination
                    return Some(*dest);
                }
            }
        }
        None
    }
}

struct FlagMoveNegativeIter<'a>(FlagMoveIter<'a>);

impl<'a> FlagMoveNegativeIter<'a> {
    fn new(components: &'a EntitySet, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        FlagMoveNegativeIter(FlagMoveIter::new(components, iter))
    }
}

impl<'a> Iterator for FlagMoveNegativeIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((src, _)) = self.0.iter.next() {
            // for each flag being moved out of src
            if self.0.components.contains(*src) {
                // if the flag was originally set
                return Some(*src);
            }
        }
        None
    }
}

struct FlagSwapNegativeIter<'a>(FlagMoveIter<'a>);

impl<'a> FlagSwapNegativeIter<'a> {
    fn new(components: &'a EntitySet, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        FlagSwapNegativeIter(FlagMoveIter::new(components, iter))
    }
}

impl<'a> Iterator for FlagSwapNegativeIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((src, dest)) = self.0.iter.next() {
            // for each flag being moved out of src
            if self.0.components.contains(*src) {
                // if the flag was originally set
                if !self.0.components.contains(*dest) {
                    // if the flag being swapped with is clear
                    return Some(*src);
                }
            }
        }
        None
    }
}

struct TypedMoveIter<'a, T: 'a> {
    components: &'a EntityMap<T>,
    iter: btree_map::Iter<'a, EntityId, EntityId>,
}

impl<'a, T: 'a> TypedMoveIter<'a, T> {
    fn new(components: &'a EntityMap<T>, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        TypedMoveIter {
            components: components,
            iter: iter,
        }
    }
}

struct TypedMovePositiveIter<'a, T: 'a>(TypedMoveIter<'a, T>);

impl<'a, T: 'a> TypedMovePositiveIter<'a, T> {
    fn new(components: &'a EntityMap<T>, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        TypedMovePositiveIter(TypedMoveIter::new(components, iter))
    }
}

impl<'a, T: 'a> Iterator for TypedMovePositiveIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((dest, src)) = self.0.iter.next() {
            // for all components being moved in
            if let Some(value) = self.0.components.get(*src) {
                // if that component has a value
                return Some((*dest, value));
            }
        }
        None
    }
}

struct TypedMoveNegativeIter<'a, T: 'a>(TypedMoveIter<'a, T>);

impl<'a, T: 'a> TypedMoveNegativeIter<'a, T> {
    fn new(components: &'a EntityMap<T>, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        TypedMoveNegativeIter(TypedMoveIter::new(components, iter))
    }
}

impl<'a, T: 'a> Iterator for TypedMoveNegativeIter<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((src, _)) = self.0.iter.next() {
            // for all components being moved out
            if self.0.components.contains_key(*src) {
                // if the component had a value to begin with
                return Some(*src);
            }
        }
        None
    }
}

// There is no positive swap iterator, as if a value is moving into
// an entity, it doesn't matter if there was already a value of that
// component present. When moving a component out of an entity, that
// component will definitely not be present after the move. With a
// swap, the presence of a value dependends on the value of the component
// in the entity being swapped with.
struct TypedSwapNegativeIter<'a, T: 'a>(TypedMoveIter<'a, T>);

impl<'a, T: 'a> TypedSwapNegativeIter<'a, T> {
    fn new(components: &'a EntityMap<T>, iter: btree_map::Iter<'a, EntityId, EntityId>) -> Self {
        TypedSwapNegativeIter(TypedMoveIter::new(components, iter))
    }
}

impl<'a, T: 'a> Iterator for TypedSwapNegativeIter<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((src, dest)) = self.0.iter.next() {
            // for all components being moved out
            if self.0.components.contains_key(*src) {
                // if the component had a value to begin with
                if !self.0.components.contains_key(*dest) {
                    // the entity being swapped with didn't have this component
                    return Some(*src);
                }
            }
        }
        None
    }
}

struct SwapTable {
    lookup: BTreeMap<EntityId, EntityId>,
    apply: Vec<(EntityId, EntityId)>,
}

impl SwapTable {
    fn new() -> Self {
        SwapTable {
            lookup: BTreeMap::new(),
            apply: Vec::new(),
        }
    }
    fn clear(&mut self) {
        self.lookup.clear();
        self.apply.clear();
    }
    fn swap(&mut self, a: EntityId, b: EntityId) {
        self.lookup.insert(a, b);
        self.lookup.insert(b, a);
        self.apply.push((a, b));
    }
    fn swaps_with(&self, entity: EntityId) -> Option<EntityId> {
        self.lookup.get(&entity).map(|r| *r)
    }
    fn typed_positive_iter<'a, T>(&'a self, components: &'a EntityMap<T>) -> TypedMovePositiveIter<'a, T> {
        TypedMovePositiveIter::new(components, self.lookup.iter())
    }
    fn typed_negative_iter<'a, T>(&'a self, components: &'a EntityMap<T>) -> TypedSwapNegativeIter<'a, T> {
        TypedSwapNegativeIter::new(components, self.lookup.iter())
    }
    fn flag_positive_iter<'a>(&'a self, components: &'a EntitySet) -> FlagMovePositiveIter<'a> {
        FlagMovePositiveIter::new(components, self.lookup.iter())
    }
    fn flag_negative_iter<'a>(&'a self, components: &'a EntitySet) -> FlagSwapNegativeIter<'a> {
        FlagSwapNegativeIter::new(components, self.lookup.iter())
    }
}

struct MoveProfile {
    source: EntityId,
    destination: EntityId,
}

impl MoveProfile {
    fn new(source: EntityId, destination: EntityId) -> Self {
        MoveProfile {
            source: source,
            destination: destination,
        }
    }
}

struct MoveTable {
    lookup_from: BTreeMap<EntityId, EntityId>,
    lookup_to: BTreeMap<EntityId, EntityId>,
    apply: Vec<MoveProfile>,
}

impl MoveTable {
    fn new() -> Self {
        MoveTable {
            lookup_from: BTreeMap::new(),
            lookup_to: BTreeMap::new(),
            apply: Vec::new(),
        }
    }
    fn moves_in(&self, entity: EntityId) -> Option<EntityId> {
        self.lookup_to.get(&entity).map(|r| *r)
    }
    fn moves_out(&self, entity: EntityId) -> Option<EntityId> {
        self.lookup_from.get(&entity).map(|r| *r)
    }
    fn clear(&mut self) {
        self.lookup_from.clear();
        self.lookup_to.clear();
        self.apply.clear();
    }
    fn mv(&mut self, source: EntityId, destination: EntityId) {
        self.lookup_from.insert(source, destination);
        self.lookup_to.insert(destination, source);
        self.apply.push(MoveProfile::new(source, destination));
    }
    fn typed_positive_iter<'a, T>(&'a self, components: &'a EntityMap<T>) -> TypedMovePositiveIter<'a, T> {
        TypedMovePositiveIter::new(components, self.lookup_to.iter())
    }
    fn typed_negative_iter<'a, T>(&'a self, components: &'a EntityMap<T>) -> TypedMoveNegativeIter<'a, T> {
        TypedMoveNegativeIter::new(components, self.lookup_from.iter())
    }
    fn flag_positive_iter<'a>(&'a self, components: &'a EntitySet) -> FlagMovePositiveIter<'a> {
        FlagMovePositiveIter::new(components, self.lookup_to.iter())
    }
    fn flag_negative_iter<'a>(&'a self, components: &'a EntitySet) -> FlagMoveNegativeIter<'a> {
        FlagMoveNegativeIter::new(components, self.lookup_from.iter())
    }
}

pub struct FlagActionProfile {
    insertions: EntityHashSet,
    removals: EntityHashSet,
    swaps: SwapTable,
    moves: MoveTable,
    changed_entities: EntitySet,
}

impl FlagActionProfile {
    fn new() -> Self {
        FlagActionProfile {
            insertions: EntityHashSet::new(),
            removals: EntityHashSet::new(),
            swaps: SwapTable::new(),
            moves: MoveTable::new(),
            changed_entities: EntitySet::new(),
        }
    }

    fn positive_iter<'a>(&'a self, components: &'a EntitySet) -> FlagActionPositiveIter<'a> {
        FlagActionPositiveIter {
            insertion_iter: self.insertions.iter(),
            move_iter: self.moves.flag_positive_iter(components),
            swap_iter: self.swaps.flag_positive_iter(components),
        }
    }

    fn negative_iter<'a>(&'a self, components: &'a EntitySet) -> FlagActionNegativeIter<'a> {
        FlagActionNegativeIter {
            removal_iter: self.removals.iter(),
            move_iter: self.moves.flag_negative_iter(components),
            swap_iter: self.swaps.flag_negative_iter(components),
        }
    }

    fn clear(&mut self) {
        self.insertions.clear();
        self.removals.clear();
        self.swaps.clear();
        self.moves.clear();
        self.changed_entities.clear();
    }

    pub fn insertion_iter(&self) -> EntityHashSetIter {
        self.insertions.iter()
    }

    pub fn removal_iter(&self) -> EntityHashSetIter {
        self.removals.iter()
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.insertions.contains(id)
    }
}

pub struct FlagActionPositiveIter<'a> {
    insertion_iter: EntityHashSetIter<'a>,
    move_iter: FlagMovePositiveIter<'a>,
    swap_iter: FlagMovePositiveIter<'a>,
}

impl<'a> Iterator for FlagActionPositiveIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.insertion_iter.next().or_else(
            || self.move_iter.next().or_else(
                || self.swap_iter.next()))
    }
}

pub struct FlagActionNegativeIter<'a> {
    removal_iter: EntityHashSetIter<'a>,
    move_iter: FlagMoveNegativeIter<'a>,
    swap_iter: FlagSwapNegativeIter<'a>,
}

impl<'a> Iterator for FlagActionNegativeIter<'a> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.removal_iter.next().or_else(
            || self.move_iter.next().or_else(
                || self.swap_iter.next()))
    }
}

pub struct TypedActionProfile<T> {
    insertions: EntityHashMap<T>,
    removals: EntityHashSet,
    swaps: SwapTable,
    moves: MoveTable,
    changed_entities: EntitySet,
}

impl<T> TypedActionProfile<T> {
     fn new() -> Self {
        TypedActionProfile {
            insertions: EntityHashMap::new(),
            removals: EntityHashSet::new(),
            swaps: SwapTable::new(),
            moves: MoveTable::new(),
            changed_entities: EntitySet::new(),
        }
    }

    pub fn insertion_iter(&self) -> EntityHashMapIter<T> {
        self.insertions.iter()
    }

    pub fn removal_iter(&self) -> EntityHashSetIter {
        self.removals.iter()
    }

    fn positive_iter<'a>(&'a self, components: &'a EntityMap<T>) -> TypedActionPositiveIter<'a, T> {
        TypedActionPositiveIter {
            insertion_iter: self.insertions.iter(),
            move_iter: self.moves.typed_positive_iter(components),
            swap_iter: self.swaps.typed_positive_iter(components),
        }
    }

    fn negative_iter<'a>(&'a self, components: &'a EntityMap<T>) -> TypedActionNegativeIter<'a, T> {
        TypedActionNegativeIter {
            removal_iter: self.removals.iter(),
            move_iter: self.moves.typed_negative_iter(components),
            swap_iter: self.swaps.typed_negative_iter(components),
        }
    }

    fn clear(&mut self) {
        self.insertions.clear();
        self.removals.clear();
        self.swaps.clear();
        self.moves.clear();
        self.changed_entities.clear();
    }

    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.insertions.get(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.insertions.get_mut(id)
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.insertions.contains_key(id)
    }
}

impl<T: Copy> TypedActionProfile<T> {
    pub fn insertion_copy_iter(&self) -> EntityHashMapCopyIter<T> {
        self.insertions.copy_iter()
    }

    pub fn get_copy(&self, id: EntityId) -> Option<T> {
        self.get(id).map(|r| *r)
    }
}
pub struct TypedActionPositiveIter<'a, T: 'a> {
    insertion_iter: EntityHashMapIter<'a, T>,
    move_iter: TypedMovePositiveIter<'a, T>,
    swap_iter: TypedMovePositiveIter<'a, T>,
}

impl<'a, T: 'a> Iterator for TypedActionPositiveIter<'a, T> {
    type Item = (EntityId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.insertion_iter.next().or_else(
            || self.move_iter.next().or_else(
                || self.swap_iter.next()))
    }
}

pub struct TypedActionNegativeIter<'a, T: 'a> {
    removal_iter: EntityHashSetIter<'a>,
    move_iter: TypedMoveNegativeIter<'a, T>,
    swap_iter: TypedSwapNegativeIter<'a, T>,
}

impl<'a, T: 'a> Iterator for TypedActionNegativeIter<'a, T> {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.removal_iter.next().or_else(
            || self.move_iter.next().or_else(
                || self.swap_iter.next()))
    }
}

pub struct EcsAction {

{{#each component}}
    {{#if type}}
    {{id}}: TypedActionProfile<{{type}}>,
    {{else}}
    {{id}}: FlagActionProfile,
    {{/if}}
{{/each}}
    changed_components: ComponentTypeSet,
    properties: EcsActionProperties,
}

impl Default for EcsAction {
    fn default() -> Self {
        Self::new()
    }
}

impl EcsAction {
    pub fn new() -> Self {
        EcsAction {
{{#each component}}
    {{#if type}}
            {{id}}: TypedActionProfile::new(),
    {{else}}
            {{id}}: FlagActionProfile::new(),
    {{/if}}
{{/each}}
            changed_components: ComponentTypeSet::new(),
            properties: EcsActionProperties::new(),
        }
    }

    pub fn clear(&mut self) {
{{#each component}}
        if self.changed_components.contains_{{id}}() {
            self.{{id}}.clear();
        }
{{/each}}
        self.changed_components.clear();
        self.properties.clear();
    }

{{#each component}}
    {{#if type}}
    pub fn insert_{{id}}(&mut self, entity: EntityId, value: {{type}}) {
        self.{{id}}.insertions.insert(entity, value);
        self.{{id}}.changed_entities.insert(entity);
        self.changed_components.insert_{{id}}();
    }
    pub fn {{id}}_profile(&self) -> &TypedActionProfile<{{type}}> {
        &self.{{id}}
    }
    pub fn {{id}}_profile_mut(&mut self) -> &mut TypedActionProfile<{{type}}> {
        &mut self.{{id}}
    }
    pub fn {{id}}_mut(&mut self, id: EntityId) -> Option<&mut {{type}}> {
        self.{{id}}_profile_mut().get_mut(id)
    }
    pub fn contains_{{id}}(&self, id: EntityId) -> bool {
        self.{{id}}_profile().contains(id)
    }
        {{#if copy}}
    pub fn {{id}}(&self, id: EntityId) -> Option<{{type}}> {
        self.{{id}}_ref(id).map(|r| *r)
    }
    pub fn {{id}}_ref(&self, id: EntityId) -> Option<&{{type}}> {
        self.{{id}}_profile().get(id)
    }
        {{else}}
    pub fn {{id}}(&self, id: EntityId) -> Option<&{{type}}> {
        self.{{id}}_profile().get(id)
    }
        {{/if}}
        {{#unless container}}
    pub fn {{id}}_positive_iter<'a>(&'a self, ecs: &'a EcsCtx) -> TypedActionPositiveIter<'a, {{type}}> {
        self.{{id}}.positive_iter(&ecs.{{id}})
    }
    pub fn {{id}}_negative_iter<'a>(&'a self, ecs: &'a EcsCtx) -> TypedActionNegativeIter<'a, {{type}}> {
        self.{{id}}.negative_iter(&ecs.{{id}})
    }
        {{/unless}}
    {{else}}
    pub fn contains_{{id}}(&self, id: EntityId) -> bool {
        self.{{id}}.contains(id)
    }
    pub fn {{id}}_positive_iter<'a>(&'a self, ecs: &'a EcsCtx) -> FlagActionPositiveIter<'a> {
        self.{{id}}.positive_iter(&ecs.{{id}})
    }
    pub fn {{id}}_negative_iter<'a>(&'a self, ecs: &'a EcsCtx) -> FlagActionNegativeIter<'a> {
        self.{{id}}.negative_iter(&ecs.{{id}})
    }
    pub fn insert_{{id}}(&mut self, entity: EntityId) {
        self.{{id}}.insertions.insert(entity);
        self.{{id}}.changed_entities.insert(entity);
        self.changed_components.insert_{{id}}();
    }
    pub fn {{id}}_profile(&self) -> &FlagActionProfile {
        &self.{{id}}
    }
    {{/if}}
    pub fn remove_{{id}}(&mut self, entity: EntityId) {
        self.{{id}}.removals.insert(entity);
        self.{{id}}.changed_entities.insert(entity);
        self.changed_components.insert_{{id}}();
    }
    pub fn move_{{id}}(&mut self, source: EntityId, destination: EntityId) {
        self.{{id}}.moves.mv(source, destination);
        self.{{id}}.changed_entities.insert(source);
        self.{{id}}.changed_entities.insert(destination);
        self.changed_components.insert_{{id}}();
    }
    pub fn swap_{{id}}(&mut self, a: EntityId, b: EntityId) {
        self.{{id}}.swaps.swap(a, b);
        self.{{id}}.changed_entities.insert(a);
        self.{{id}}.changed_entities.insert(b);
        self.changed_components.insert_{{id}}();
    }
{{/each}}
    pub fn remove_entity(&mut self, entity: EntityRef) {
        self.remove_entity_by_id(entity.id, entity.ctx);
    }
    pub fn remove_entity_by_id(&mut self, entity: EntityId, ecs: &EcsCtx) {
        ecs.tracker.get(entity).map(|components| {
            for component in components.iter() {
                self.remove_component(entity, component);
            }
        });
    }
    pub fn remove_component(&mut self, entity: EntityId, component_type: ComponentType) {
        match component_type {
{{#each component}}
            component_type::{{id_uppercase}} => self.remove_{{id}}(entity),
{{/each}}
            _ => panic!("Invalid component type: {}", component_type),
        }
    }
{{#each action_property}}
    {{#if type}}
        {{#if copy}}
    pub fn {{id}}(&self) -> Option<{{type}}> {
        self.properties.{{id}}()
    }
    pub fn {{id}}_ref(&self) -> Option<&{{type}}> {
        self.properties.{{id}}_ref()
    }
        {{else}}
    pub fn {{id}}(&self) -> Option<&{{type}}> {
        self.properties.{{id}}()
    }
        {{/if}}
    pub fn set_{{id}}(&mut self, value: {{type}}) {
        self.properties.insert_{{id}}(value);
    }
    {{else}}
    pub fn contains_{{id}}(&self) -> bool {
        self.properties.contains_{{id}}()
    }
    pub fn set_{{id}}(&mut self) {
        self.properties.insert_{{id}}();
    }
    {{/if}}
    pub fn clear_{{id}}(&mut self) ->
    {{#if type}}
        Option<{{type}}>
    {{else}}
        bool
    {{/if}}
    {
        self.properties.remove_{{id}}()
    }
{{/each}}

    pub fn entity(&self, id: EntityId) -> ActionEntityRef {
        ActionEntityRef::new(id, self)
    }

    pub fn entity_mut(&mut self, id: EntityId) -> ActionEntityRefMut {
        ActionEntityRefMut::new(id, self)
    }
}

#[derive(Clone, Copy)]
pub enum ContainerComponentRef<'a, ContainerType: 'a, Type: 'a> {
    Contained(&'a ContainerType),
    Raw(&'a Type),
}

#[derive(Clone, Copy)]
pub enum Change<T> {
    Insert(T),
    Remove,
}

#[derive(Clone, Copy)]
pub struct PostActionEntityRef<'a> {
    id: EntityId,
    ecs: &'a EcsCtx,
    action: &'a EcsAction,
}

impl<'a> PostActionEntityRef<'a> {
    fn new(id: EntityId, ecs: &'a EcsCtx, action: &'a EcsAction) -> Self {
        PostActionEntityRef {
            id: id,
            ecs: ecs,
            action: action,
        }
    }

    pub fn id(self) -> EntityId {
        self.id
    }

    pub fn to_entity_ref(self) -> EntityRef<'a> {
        EntityRef::new(self.id, self.ecs)
    }

{{#each component}}
    {{#if type}}
        {{#if container}}
    pub fn {{id}}(self) -> Option<ContainerComponentRef<'a, {{container}}<{{type}}>, {{type}}>> {
        if let Some(change) = self.change_{{id}}() {
            if let Change::Insert(component) = change {
                Some(component)
            } else {
                None
            }
        } else {
            self.current_{{id}}().map(ContainerComponentRef::Contained)
        }
    }
    pub fn current_{{id}}(self) -> Option<&'a {{container}}<{{type}}>> {
        self.ecs.{{id}}(self.id)
    }
    pub fn change_{{id}}(self) -> Option<Change<ContainerComponentRef<'a, {{container}}<{{type}}>, {{type}}>>> {
        let profile = &self.action.{{id}};

        // check if component is explicitly inserted
        if let Some(component) = profile.insertions.get(self.id) {
            return Some(Change::Insert(ContainerComponentRef::Raw(component)));
        }

        // check if component is explicitly removed
        if profile.removals.contains(self.id) {
            return Some(Change::Remove);
        }

        // check if component is swapped in
        if let Some(other) = profile.swaps.swaps_with(self.id) {
            if let Some(component) = self.ecs.{{id}}(other) {
                return Some(Change::Insert(ContainerComponentRef::Contained(component)));
            } else {
                return Some(Change::Remove);
            }
        }

        // check if component is moved in
        if let Some(src) = profile.moves.moves_in(self.id) {
            if let Some(component) = self.ecs.{{id}}(src) {
                return Some(Change::Insert(ContainerComponentRef::Contained(component)));
            }
        }

        // check if component is moved out
        if profile.moves.moves_out(self.id).is_some() {
            return Some(Change::Remove);
        }

        None
    }
        {{else}}
            {{#if copy}}
    pub fn {{id}}(self) -> Option<{{type}}> {
        if let Some(change) = self.change_{{id}}() {
            if let Change::Insert(component) = change {
                Some(component)
            } else {
                None
            }
        } else {
            self.current_{{id}}()
        }
    }
    pub fn current_{{id}}(self) -> Option<{{type}}> {
        self.ecs.{{id}}(self.id)
    }
    pub fn change_{{id}}(self) -> Option<Change<{{type}}>> {
        let profile = &self.action.{{id}};

        // check if component is explicitly inserted
        if let Some(component) = profile.insertions.get(self.id) {
            return Some(Change::Insert(*component));
        }

        // check if component is explicitly removed
        if profile.removals.contains(self.id) {
            return Some(Change::Remove);
        }

        // check if component is swapped in
        if let Some(other) = profile.swaps.swaps_with(self.id) {
            if let Some(component) = self.ecs.{{id}}(other) {
                return Some(Change::Insert(component));
            } else {
                return Some(Change::Remove);
            }
        }

        // check if component is moved in
        if let Some(src) = profile.moves.moves_in(self.id) {
            if let Some(component) = self.ecs.{{id}}(src) {
                return Some(Change::Insert(component));
            }
        }

        // check if component is moved out
        if profile.moves.moves_out(self.id).is_some() {
            return Some(Change::Remove);
        }

        None
    }
    pub fn {{id}}_ref(self) -> Option<&'a {{type}}> {
        if let Some(change) = self.change_{{id}}_ref() {
            if let Change::Insert(component) = change {
                Some(component)
            } else {
                None
            }
        } else {
            self.current_{{id}}_ref()
        }
    }
    pub fn current_{{id}}_ref(self) -> Option<&'a {{type}}> {
        self.ecs.{{id}}_ref(self.id)
    }
    pub fn change_{{id}}_ref(self) -> Option<Change<&'a {{type}}>> {
        let profile = &self.action.{{id}};

        // check if component is explicitly inserted
        if let Some(component) = profile.insertions.get(self.id) {
            return Some(Change::Insert(component));
        }

        // check if component is explicitly removed
        if profile.removals.contains(self.id) {
            return Some(Change::Remove);
        }

        // check if component is swapped in
        if let Some(other) = profile.swaps.swaps_with(self.id) {
            if let Some(component) = self.ecs.{{id}}_ref(other) {
                return Some(Change::Insert(component));
            } else {
                return Some(Change::Remove);
            }
        }

        // check if component is moved in
        if let Some(src) = profile.moves.moves_in(self.id) {
            if let Some(component) = self.ecs.{{id}}_ref(src) {
                return Some(Change::Insert(component));
            }
        }

        // check if component is moved out
        if profile.moves.moves_out(self.id).is_some() {
            return Some(Change::Remove);
        }

        None
    }
            {{else}}
    pub fn {{id}}(self) -> Option<&'a {{type}}> {
        if let Some(change) = self.change_{{id}}() {
            if let Change::Insert(component) = change {
                Some(component)
            } else {
                None
            }
        } else {
            self.current_{{id}}()
        }
    }
    pub fn current_{{id}}(self) -> Option<&'a {{type}}> {
        self.ecs.{{id}}(self.id)
    }
    pub fn change_{{id}}(self) -> Option<Change<&'a {{type}}>> {
        let profile = &self.action.{{id}};

        // check if component is explicitly inserted
        if let Some(component) = profile.insertions.get(self.id) {
            return Some(Change::Insert(component));
        }

        // check if component is explicitly removed
        if profile.removals.contains(self.id) {
            return Some(Change::Remove);
        }

        // check if component is swapped in
        if let Some(other) = profile.swaps.swaps_with(self.id) {
            if let Some(component) = self.ecs.{{id}}(other) {
                return Some(Change::Insert(component));
            } else {
                return Some(Change::Remove);
            }
        }

        // check if component is moved in
        if let Some(src) = profile.moves.moves_in(self.id) {
            if let Some(component) = self.ecs.{{id}}(src) {
                return Some(Change::Insert(component));
            }
        }

        // check if component is moved out
        if profile.moves.moves_out(self.id).is_some() {
            return Some(Change::Remove);
        }

        None
    }
            {{/if}}
        {{/if}}
    {{else}}
    pub fn contains_{{id}}(self) -> bool {
        self.change_{{id}}().unwrap_or_else(|| self.current_contains_{{id}}())
    }

    pub fn current_contains_{{id}}(self) -> bool {
        self.ecs.contains_{{id}}(self.id)
    }

    // returns true iff the flag will be set after this action
    pub fn change_{{id}}(self) -> Option<bool> {
        let profile = &self.action.{{id}};

        // check if component is explicitly inserted
        if profile.insertions.contains(self.id) {
            return Some(true);
        }

        // check if component is explicitly removed
        if profile.removals.contains(self.id) {
            return Some(false);
        }

        // check if component is swapped in or out
        if let Some(other) = profile.swaps.swaps_with(self.id) {
            if self.ecs.contains_{{id}}(other) {
                return Some(true);
            } else {
                return Some(false);
            }
        }

        // check if component is moved in
        if let Some(src) = profile.moves.moves_in(self.id) {
            if self.ecs.contains_{{id}}(src) {
                return Some(true);
            }
        }

        // check if component is moved out
        if profile.moves.moves_out(self.id).is_some() {
            return Some(false);
        }

        None
    }
    {{/if}}
{{/each}}
}

pub struct ActionEntityRefMut<'a> {
    id: EntityId,
    action: &'a mut EcsAction,
}

impl<'a> ActionEntityRefMut<'a> {
    fn new(id: EntityId, action: &'a mut EcsAction) -> Self {
        ActionEntityRefMut {
            id: id,
            action: action,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

{{#each component}}
    pub fn remove_{{id}}(&mut self) {
        self.action.remove_{{id}}(self.id);
    }

    {{#if type}}
    pub fn {{id}}_mut(&mut self, id: EntityId) -> Option<&mut {{type}}> {
        self.action.{{id}}_mut(id)
    }
    {{/if}}
{{/each}}
}

pub struct ActionEntityRef<'a> {
    id: EntityId,
    action: &'a EcsAction,
}

impl<'a> ActionEntityRef<'a> {
    fn new(id: EntityId, action: &'a EcsAction) -> Self {
        ActionEntityRef {
            id: id,
            action: action,
        }
    }

    pub fn id(self) -> EntityId {
        self.id
    }

{{#each component}}
    pub fn contains_{{id}}(self) -> bool {
        self.action.contains_{{id}}(self.id)
    }
    {{#if type}}
        {{#if copy}}
    pub fn {{id}}(self) -> Option<{{type}}> {
        self.action.{{id}}(self.id)
    }
    pub fn {{id}}_ref(self) -> Option<&'a {{type}}> {
        self.action.{{id}}_ref(self.id)
    }
        {{else}}
    pub fn {{id}}(self) -> Option<&'a {{type}}> {
        self.action.{{id}}(self.id)
    }
        {{/if}}
    {{/if}}
{{/each}}
}

pub const NUM_ACTION_PROPERTIES: usize = {{num_action_properties}};
const ACTION_PROPERTY_TYPE_SET_NUM_WORDS: usize = {{component_set_num_words}};

pub type ActionPropertyType = usize;

pub mod action_property_type {
    use std::usize;

{{#each action_property}}
    pub const {{id_uppercase}}: usize = {{index}};
{{/each}}
}

pub struct ActionPropertyTypeSet {
    bitfields: [usize; ACTION_PROPERTY_TYPE_SET_NUM_WORDS],
}

pub struct ActionPropertyTypeSetIter {
    bitfields: [usize; ACTION_PROPERTY_TYPE_SET_NUM_WORDS],
    index: usize,
}

impl ActionPropertyTypeSetIter {
    fn new(bitfields: [usize; ACTION_PROPERTY_TYPE_SET_NUM_WORDS]) -> Self {
        ActionPropertyTypeSetIter {
            bitfields: bitfields,
            index: 0,
        }
    }
}

impl Iterator for ActionPropertyTypeSetIter {
    type Item = ActionPropertyType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < ACTION_PROPERTY_TYPE_SET_NUM_WORDS && self.bitfields[self.index] == 0 {
            self.index += 1;
        }
        if self.index == ACTION_PROPERTY_TYPE_SET_NUM_WORDS {
            return None;
        }

        let trailing = self.bitfields[self.index].trailing_zeros() as usize;
        self.bitfields[self.index] &= !(1 << trailing);
        Some(self.index * WORD_BITS + trailing)
    }
}

impl ActionPropertyTypeSet {
    pub fn new() -> Self {
        ActionPropertyTypeSet {
            bitfields: [0; COMPONENT_TYPE_SET_NUM_WORDS],
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

    pub fn iter(&self) -> ActionPropertyTypeSetIter {
        ActionPropertyTypeSetIter::new(self.bitfields)
    }

{{#each action_property}}
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

pub struct EcsActionProperties {
    property_types: ActionPropertyTypeSet,
{{#each action_property}}
    {{#if type}}
    pub {{id}}: Option<{{type}}>,
    {{else}}
    pub {{id}}: bool,
    {{/if}}
{{/each}}
}

impl EcsActionProperties {
    pub fn new() -> Self {
        EcsActionProperties {
            property_types: ActionPropertyTypeSet::new(),
{{#each action_property}}
    {{#if type}}
            {{id}}: None,
    {{else}}
            {{id}}: false,
    {{/if}}
{{/each}}
        }
    }

    pub fn clear(&mut self) {
        for property_type in self.property_types.iter() {
            match property_type {
{{#each action_property}}
                action_property_type::{{id_uppercase}} => {
    {{#if type}}
                    self.{{id}} = None;
    {{else}}
                    self.{{id}} = false;
    {{/if}}
                }
{{/each}}
                _ => panic!("Invalid action property type: {}", property_type),
            }
        }
        self.property_types.clear();
    }

{{#each action_property}}
    {{#if type}}
    pub fn insert_{{id}}(&mut self, value: {{type}}) {
        self.{{id}} = Some(value);
        self.property_types.insert_{{id}}();
    }
        {{#if copy}}
    pub fn {{id}}(&self) -> Option<{{type}}> {
        self.{{id}}
    }
    pub fn {{id}}_ref(&self) -> Option<&{{type}}> {
        self.{{id}}.as_ref()
    }
        {{else}}
    pub fn {{id}}(&self) -> Option<&{{type}}> {
        self.{{id}}.as_ref()
    }
        {{/if}}
    pub fn contains_{{id}}(&self) -> bool {
        self.{{id}}.is_some()
    }
    {{else}}
    pub fn insert_{{id}}(&mut self) {
        self.{{id}} = true;
        self.property_types.insert_{{id}}();
    }
    pub fn contains_{{id}}(&self) -> bool {
        self.{{id}}
    }
    {{/if}}
    pub fn remove_{{id}}(&mut self) ->
    {{#if type}}
        Option<{{type}}>
    {{else}}
        bool
    {{/if}}
    {
    {{#if type}}
        mem::replace(&mut self.{{id}}, None)
    {{else}}
        mem::replace(&mut self.{{id}}, false)
    {{/if}}
    }
{{/each}}
}

pub struct EntityRefIter<'a, I: Iterator<Item=EntityId>> {
    ctx: &'a EcsCtx,
    iter: I,
}

impl<'a, I: Iterator<Item=EntityId>> EntityRefIter<'a, I> {
    fn new(ctx: &'a EcsCtx, iter: I) -> Self {
        EntityRefIter {
            ctx: ctx,
            iter: iter,
        }
    }
}

impl<'a, I: Iterator<Item=EntityId>> Iterator for EntityRefIter<'a, I> {
    type Item = EntityRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| self.ctx.entity(id))
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

    let mut index = 0;
    for (id, component) in json.as_object_mut().unwrap().get_mut("component").unwrap().as_object_mut().unwrap().iter_mut() {
        let component_obj = component.as_object_mut().unwrap();
        component_obj.insert("index".to_string(), Json::U64(index as u64));
        component_obj.insert("set_index".to_string(), Json::U64((index / word_bits) as u64));
        component_obj.insert("set_bit".to_string(), Json::U64((index % word_bits) as u64));
        component_obj.insert("id".to_string(), Json::String(id.to_string()));
        component_obj.insert("id_uppercase".to_string(), Json::String(id.to_uppercase()));

        let maybe_container = component_obj.get("container").map(|container| {
            container.clone()
        });

        if let Some(container) = maybe_container {
            component_obj.insert(container.as_string().unwrap().to_string(), Json::Boolean(true));
        }

        component_clones.insert(id.to_string(), component_obj.clone());

        index += 1;
    }

    let num_action_properties = if let Some(action_property) = json.search("action_property") {
        action_property.as_object().unwrap().len()
    } else {
        0
    };

    json.as_object_mut().unwrap().insert("num_action_properties".to_string(), Json::U64(num_action_properties as u64));
    index = 0;

    if let Some(mut action_property) = json.as_object_mut().unwrap().get_mut("action_property") {
        for (id, action_property) in action_property.as_object_mut().unwrap().iter_mut() {
            let action_property_obj = action_property.as_object_mut().unwrap();
            action_property_obj.insert("index".to_string(), Json::U64(index as u64));
            action_property_obj.insert("set_index".to_string(), Json::U64((index / word_bits) as u64));
            action_property_obj.insert("set_bit".to_string(), Json::U64((index % word_bits) as u64));
            action_property_obj.insert("id".to_string(), Json::String(id.to_string()));
            action_property_obj.insert("id_uppercase".to_string(), Json::String(id.to_uppercase()));

            index += 1;
        }
    }

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
