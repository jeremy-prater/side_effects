use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct AnimationLibrary(HashMap<String, HashMap<String, Handle<AnimationClip>>>);

impl AnimationLibrary {
    pub fn insert(
        &mut self,
        collection_name: &str,
        animation_name: &str,
        animation_clip: Handle<AnimationClip>,
    ) {
        if !self.0.contains_key(collection_name) {
            let mut new_collection: HashMap<String, Handle<AnimationClip>> = HashMap::new();
            new_collection.insert(animation_name.to_owned(), animation_clip);
            self.0.insert(collection_name.to_owned(), new_collection);
        } else {
            let target_collection = self.0.get_mut(collection_name).unwrap();
            target_collection.insert(animation_name.to_owned(), animation_clip);
        }
    }

    pub fn get(
        &self,
        collection_name: &str,
        animation_name: &str,
    ) -> Option<Handle<AnimationClip>> {
        if !self.0.contains_key(collection_name) {
            None
        } else {
            let target_collection = self.0.get(collection_name).unwrap();
            if !target_collection.contains_key(animation_name) {
                None
            } else {
                Some(target_collection.get(animation_name).unwrap().clone_weak())
            }
        }
    }
}
