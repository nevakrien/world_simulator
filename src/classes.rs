use std::hash::Hash;
use std::collections::{HashSet,HashMap};
use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct DuplicateDef;

//we assume 64bit word size
pub type ClassID = u32;
pub type PropertyID = u32;

pub trait TypeRegistery<'code>{
    fn get_class(&self,id:ClassID) -> Option<&ClassMeta<'code>>{
        self.get_class_and_name(id).map(|x| x.0)
    }
    fn get_type(&self,name:&str) -> Option<Type>;
    fn get_property(&self,id:PropertyID) -> Option<&Property>{
        self.get_property_and_name(id).map(|x| x.0)

    }

    fn get_class_id(&self,name:&str) -> Option<ClassID>;
    fn get_property_id(&self,name:&str,class:ClassID) -> Option<PropertyID>;

    fn add_class_id(&mut self,name:&'code str) -> ClassID;
    fn add_property_id(&mut self,name:&'code str,class:ClassID) -> PropertyID;

    fn add_class(&mut self,id:ClassID,value:ClassMeta<'code>) -> Result<(),DuplicateDef>;
    fn add_property(&mut self,id:PropertyID,value:Property) -> Result<(),DuplicateDef>;

    fn get_class_and_name(&self,id:ClassID) -> Option<(&ClassMeta<'code>,&'code str)>;
    fn get_property_and_name(&self,id:PropertyID) -> Option<(&Property,&'code str)>;


}


#[repr(u32)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub enum Type{
    Int=0,
    Float=1,
    String=2,
    Class(ClassID)=3,

    #[default] 
    Invalid=4,
}

impl Type{
    #[inline]
    pub fn is_valid(self) -> bool {
        match self{
            Type::Invalid => false,
            _ => true,
        }
    }
}

impl From<Type> for u64 {
	#[inline(always)]
    fn from(t: Type) -> Self {
        match t {
            Type::Int => 0u64,
            Type::Float => 1u64,
            Type::String => 2u64,
            // Shift the ClassID up by 32 bits to move it completely out of the discriminant range
            Type::Class(id) => 3u64 | ((id as u64) << 32),
            
            Type::Invalid => 4,
        }
    }
}

impl From<Type> for usize {
    fn from(t: Type) -> Self {
        u64::from(t) as usize
    }
}


#[cfg(test)]
mod layout_tests {
    use super::*;
    use std::mem;
    

 //    //this test is UB since there isnt actually a disant way to test this...
 //    //it is configured not to run with miri so testing with miri should be fine
 //    #[test]
	// #[cfg(not(miri))]
 //    fn test_type_representation_expectation() {
 //        // Test primitive variants
 //        let int_type = Type::Int;
 //        let float_type = Type::Float;
 //        let string_type = Type::String;
        
 //        // Test Class variants with different IDs
 //        let class_0 = Type::Class(0);
 //        let class_1 = Type::Class(1);
 //        let class_42 = Type::Class(42);
 //        let class_max = Type::Class(u32::MAX);
        
 //        // Safer approach using transmute
 //        // This is still questionable in terms of strict aliasing rules,
 //        // but more likely to pass Miri than raw pointer casting
 //        unsafe {
 //            let int_raw: u64 = mem::transmute(int_type);
 //            let float_raw: u64 = mem::transmute(float_type);
 //            let string_raw: u64 = mem::transmute(string_type);
 //            let class_0_raw: u64 = mem::transmute(class_0);
 //            let class_1_raw: u64 = mem::transmute(class_1);
 //            let class_42_raw: u64 = mem::transmute(class_42);
 //            let class_max_raw: u64 = mem::transmute(class_max);
            
 //            // Compare raw representation with From implementation
 //            assert_eq!(int_raw, u64::from(int_type), "Int raw memory doesn't match From<Type>");
 //            assert_eq!(float_raw, u64::from(float_type), "Float raw memory doesn't match From<Type>");
 //            assert_eq!(string_raw, u64::from(string_type), "String raw memory doesn't match From<Type>");
 //            assert_eq!(class_0_raw, u64::from(class_0), "Class(0) raw memory doesn't match From<Type>");
 //            assert_eq!(class_1_raw, u64::from(class_1), "Class(1) raw memory doesn't match From<Type>");
 //            assert_eq!(class_42_raw, u64::from(class_42), "Class(42) raw memory doesn't match From<Type>");
 //            assert_eq!(class_max_raw, u64::from(class_max), "Class(MAX) raw memory doesn't match From<Type>");
            
 //            println!("Memory layout:");
 //            println!("  Int      = 0x{:016x}", int_raw);
 //            println!("  Float    = 0x{:016x}", float_raw);
 //            println!("  String   = 0x{:016x}", string_raw);
 //            println!("  Class(0) = 0x{:016x}", class_0_raw);
 //            println!("  Class(1) = 0x{:016x}", class_1_raw);
 //            println!("  Class(42)= 0x{:016x}", class_42_raw);
 //            println!("  Class(MAX)= 0x{:016x}", class_max_raw);
            
 //            // Extract ClassID from memory representation
 //            let extract_class_id = |raw: u64| -> u32 {
 //                if (raw & 0x3) == 3 { // Check if it's a Class variant
 //                    ((raw >> 32) & 0xFFFFFFFF) as u32
 //                } else {
 //                    panic!("Not a Class variant")
 //                }
 //            };
            
 //            // Verify we can extract the ClassID directly from memory
 //            assert_eq!(extract_class_id(class_0_raw), 0);
 //            assert_eq!(extract_class_id(class_1_raw), 1);
 //            assert_eq!(extract_class_id(class_42_raw), 42);
 //            assert_eq!(extract_class_id(class_max_raw), u32::MAX);
 //        }
 //    }
    
    
    #[test]
    fn test_enum_size() {
        assert_eq!(mem::size_of::<Type>(), 8, "Type should be exactly 8 bytes");
    }
}


/// A struct that manages registration of classes and properties in the simulation system
/// using in-memory hash maps
#[derive(Debug, Default)]
pub struct InMemoryRegistry<'code> {
    // Maps class IDs to their metadata and names
    classes: HashMap<ClassID, (ClassMeta<'code>, &'code str)>,
    // Maps property IDs to their data and names
    properties: HashMap<PropertyID, (Property, &'code str)>,
    // Maps names to class IDs for quick lookup
    class_names: HashMap<&'code str, ClassID>,
    // Maps names to property IDs for quick lookup
    property_names: HashMap<&'code str, HashMap<ClassID,PropertyID>>,
    // Counters for generating new IDs
    next_class_id: ClassID,
    next_property_id: PropertyID,
}

impl<'code> InMemoryRegistry<'code> {
    /// Creates a new empty registry
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            properties: HashMap::new(),
            class_names: HashMap::new(),
            property_names: HashMap::new(),
            next_class_id: 1, // Starting IDs from 1, 0 could be reserved
            next_property_id: 1,
        }
    }
}

impl<'code> TypeRegistery<'code> for InMemoryRegistry<'code> {
    fn get_type(&self, name: &str) -> Option<Type> {
        match name {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            _ => self.get_class_id(name).map(Type::Class),
        }
    }

    fn get_class_id(&self, name: &str) -> Option<ClassID> {
        self.class_names.get(name).copied()
    }

    fn get_property_id(&self, name: &str,class:ClassID) -> Option<PropertyID> {
        self.property_names.get(name).and_then(|x| x.get(&class).copied())
    }

    fn add_class_id(&mut self, name: &'code str) -> ClassID {
        if let Some(id) = self.get_class_id(name) {
            return id;
        }
        
        let id = self.next_class_id;
        self.next_class_id = self.next_class_id + 1;
        self.class_names.insert(name, id);
        id
    }

    fn add_property_id(&mut self, name: &'code str,class:ClassID) -> PropertyID {
        
        let id = self.next_property_id;
        self.next_property_id = self.next_property_id + 1;
        // self.property_names.insert(name, id);
        if self.property_names.entry(name)
        .or_default()
        .insert(class,id)
        .is_some() {
            panic!("duplicate properties on class!!!");
        }

        match self.properties.entry(id) {
            Entry::Occupied(_) => panic!("duplicate property ID added"),
            Entry::Vacant(spot) => spot.insert((Property::default(),name)),
        };

        id
    }

    fn add_class(&mut self, id: ClassID, value: ClassMeta<'code>) -> Result<(), DuplicateDef> {
        match self.classes.entry(id) {
            Entry::Occupied(_) => Err(DuplicateDef),
            Entry::Vacant(entry) => {
                // We need the name for this class ID
                let name = self.class_names.iter()
                    .find_map(|(&name, &class_id)| if class_id == id { Some(name) } else { None })
                    .ok_or(DuplicateDef)?;
                entry.insert((value, name));
                Ok(())
            }
        }
    }

    fn add_property(&mut self, id: PropertyID, value: Property) -> Result<(), DuplicateDef> {
        match self.properties.entry(id) {
            Entry::Occupied(mut spot) => {
                let v  = &mut spot.get_mut().0;
                if !v.inner_type.is_valid() {
                    *v=value;
                    Ok(())
                }else{
                    Err(DuplicateDef)
                }

            },
            Entry::Vacant(_) => {
                panic!("tried adding a non existed property id");
            }
        }
    }

    fn get_class_and_name(&self, id: ClassID) -> Option<(&ClassMeta<'code>, &'code str)> {
        self.classes.get(&id).map(|(meta, name)| (meta, *name))
    }

    fn get_property_and_name(&self, id: PropertyID) -> Option<(&Property, &'code str)> {
        self.properties.get(&id).map(|(prop, name)| (prop, *name))
    }
}



#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub struct Property{
    pub id: PropertyID,
	pub inner_type: Type,
	pub source: ClassID,
}

#[derive(Debug,Clone,PartialEq)]
pub struct ClassMeta<'code>{
    pub parents: HashSet<ClassID>,

    /// includes all possible classes this can be downcasted to
	pub ancestors: HashSet<ClassID>,

    /// properties that can be accessed via obj.name 
	pub accessble_properties: HashMap<&'code str,Property>,

    /// properties where there is more than 1 correct interpetation for which to take
	pub clashing_properties: HashMap<&'code str,HashSet<Property>>,

    /// properties hidden behind another property with the same name 
    /// this can happen when a class has a defined property that shares a name with a parents
    /// in that case the parents property is shadowed in that class
    pub shadowed_properties: HashMap<&'code str,HashSet<Property>>,
}


impl<'code> ClassMeta<'code>{
    pub fn new(reg:&impl TypeRegistery<'code>,id:ClassID,parents: HashSet<ClassID>,new_props:HashMap<&'code str,Property>)->Self{
        let mut ans = ClassMeta{
            ancestors: parents.clone(),
            parents,

            accessble_properties:new_props,

            clashing_properties: HashMap::new(),
            shadowed_properties: HashMap::new(),
        };

        for p in &ans.parents{
            let p = reg.get_class(*p).unwrap();

            ans.ancestors.extend(p.ancestors.clone());

            //once something is shadowed its allways shadowed
            for (k,v) in &p.shadowed_properties{
                ans.shadowed_properties
                .entry(k)
                .or_default()
                .extend(v);
            }

            //clashing might be downgraded to shadowed
            for (k,v) in &p.clashing_properties{
                match ans.accessble_properties.entry(k){
                    Entry::Occupied(entry) => {
                        if entry.get().id==id {
                            ans.shadowed_properties
                            .entry(k)
                            .or_default()
                            .extend(v);
                        } else{
                            //if we found another property we clash with bump it out
                            let (_,other) = entry.remove_entry();
                            let s = ans.clashing_properties
                            .entry(k)
                            .or_default();
                            
                            s.extend(v);
                            s.insert(other);
                        }
                    },
                    Entry::Vacant(_) => {
                        ans.clashing_properties
                        .entry(k)
                        .or_default()
                        .extend(v);
                    } 
                }
            }

            //accible might clash or downgrade
            for (k,v) in &p.accessble_properties{
                match ans.accessble_properties.entry(k) {
                    Entry::Occupied(entry) => {
                        let other_id = entry.get().id;
                        if  other_id==id {
                            ans.shadowed_properties
                            .entry(k)
                            .or_default()
                            .insert(*v);
                        } else if v.source==other_id {
                                continue; //its the same entry so we are good
                        }else{
                            //if we found another property we clash with bump it out
                            let (_,other) = entry.remove_entry();
                            let s = ans.clashing_properties
                            .entry(k)
                            .or_default();
                            
                            s.insert(*v);
                            s.insert(other);
                        }
                    },
                    Entry::Vacant(spot) => {
                        spot.insert(v.clone());
                    } 
                };
            }
        }

        ans
    }
}

#[cfg(test)]
mod class_meta_tests {
    use super::*;
    
    // This test module verifies the ClassMeta::new method's handling of complex inheritance cases
    // Including:
    // 1. Simple inheritance (B inherits from A)
    // 2. Diamond inheritance (D inherits from B and C, both of which inherit from A)
    // 3. Property shadowing (when a class defines a property with the same name as an inherited one)
    // 4. Property clashing (when inheriting properties with the same name from different sources)
    // 5. Multi-level inheritance (4+ levels deep)
    
    // Helper function to create a property
    fn create_property<'a>(reg: &mut InMemoryRegistry<'a>, prop_name: &'a str, class_id: ClassID, prop_type: Type) -> Property {
        let prop_id = reg.add_property_id(prop_name,class_id);
        let property = Property {
            id: prop_id,
            inner_type: prop_type,
            source: class_id,
        };
        reg.add_property(prop_id, property).unwrap();
        property
    }
    
    // Helper function to set up a class with properties
    fn setup_class<'a>(
        reg: &mut InMemoryRegistry<'a>,
        class_name: &'a str,
        parents: HashSet<ClassID>,
        properties: Vec<(&'a str, Type)>,
    ) -> ClassID {
        let class_id = reg.add_class_id(class_name);
        
        // Create the properties for this class
        let mut props_map = HashMap::new();
        for (prop_name, prop_type) in properties {
            let property = create_property(reg, prop_name, class_id, prop_type);
            props_map.insert(prop_name, property);
        }
        
        // Create the class metadata
        let class_meta = ClassMeta::new(reg, class_id, parents, props_map);
        reg.add_class(class_id, class_meta).unwrap();
        
        class_id
    }
    
    #[test]
    fn test_simple_inheritance() {
        // Test basic inheritance where B inherits from A
        let mut registry = InMemoryRegistry::new();
        
        // Create class A with properties a1 and a2
        let a_id = setup_class(
            &mut registry,
            "A",
            HashSet::new(),
            vec![("a1", Type::Int), ("a2", Type::String)],
        );
        
        // Create class B inheriting from A with property b1
        let b_id = setup_class(
            &mut registry,
            "B",
            HashSet::from([a_id]),
            vec![("b1", Type::Float)],
        );
        
        // Verify B's accessible properties include both its own and those from A
        let b_meta = registry.get_class(b_id).unwrap();
        assert!(b_meta.accessble_properties.contains_key("a1"), "B should inherit a1 from A");
        assert!(b_meta.accessble_properties.contains_key("a2"), "B should inherit a2 from A");
        assert!(b_meta.accessble_properties.contains_key("b1"), "B should have its own property b1");
        
        // Verify B's ancestors include A
        assert!(b_meta.ancestors.contains(&a_id), "B's ancestors should include A");
        
        // Verify there are no clashing or shadowed properties
        assert!(b_meta.clashing_properties.is_empty(), "There should be no clashing properties");
        assert!(b_meta.shadowed_properties.is_empty(), "There should be no shadowed properties");
    }
    
    #[test]
    fn test_property_shadowing() {
        // Test shadowing when a class defines a property with the same name as an inherited one
        let mut registry = InMemoryRegistry::new();
        
        // Create class A with property "name"
        let a_id = setup_class(
            &mut registry,
            "A",
            HashSet::new(),
            vec![("name", Type::String), ("age", Type::Int)],
        );
        
        // Create class B inheriting from A with its own "name" property
        let b_id = setup_class(
            &mut registry,
            "B",
            HashSet::from([a_id]),
            vec![("name", Type::String)], // Same name as A's property
        );
        
        // Verify B's accessible properties include "name" and "age"
        let b_meta = registry.get_class(b_id).unwrap();
        assert!(b_meta.accessble_properties.contains_key("name"), "B should have name property");
        assert!(b_meta.accessble_properties.contains_key("age"), "B should inherit age from A");
        
        // The "name" property in B should shadow A's "name" property
        assert!(b_meta.shadowed_properties.contains_key("name"), "A's name property should be shadowed");
        let shadowed = b_meta.shadowed_properties.get("name").unwrap();
        assert_eq!(shadowed.len(), 1, "There should be one shadowed name property");
        
        // The source of the accessible "name" property should be B
        let accessible_name = b_meta.accessble_properties.get("name").unwrap();
        assert_eq!(accessible_name.source, b_id, "The accessible name property should be B's own");
    }
    
    #[test]
    fn test_diamond_inheritance() {
        // Test diamond inheritance: A -> B -> D
        //                            \-> C -/
        let mut registry = InMemoryRegistry::new();
        
        // Create class A with property "a_prop"
        let a_id = setup_class(
            &mut registry,
            "A",
            HashSet::new(),
            vec![("a_prop", Type::Int)],
        );
        
        // Create class B inheriting from A with property "b_prop"
        let b_id = setup_class(
            &mut registry,
            "B",
            HashSet::from([a_id]),
            vec![("b_prop", Type::Float)],
        );
        
        // Create class C inheriting from A with property "c_prop"
        let c_id = setup_class(
            &mut registry,
            "C",
            HashSet::from([a_id]),
            vec![("c_prop", Type::String)],
        );
        
        // Create class D inheriting from both B and C
        let d_id = setup_class(
            &mut registry,
            "D",
            HashSet::from([b_id, c_id]),
            vec![("d_prop", Type::Int)],
        );
        
        // Verify D's accessible properties
        let d_meta = registry.get_class(d_id).unwrap();
        
        // D should have access to all properties from A, B, C, and its own
        assert!(d_meta.accessble_properties.contains_key("a_prop"), "D should inherit a_prop from A");
        assert!(d_meta.accessble_properties.contains_key("b_prop"), "D should inherit b_prop from B");
        assert!(d_meta.accessble_properties.contains_key("c_prop"), "D should inherit c_prop from C");
        assert!(d_meta.accessble_properties.contains_key("d_prop"), "D should have its own d_prop");
        
        // Verify D's ancestors include A, B, and C
        assert!(d_meta.ancestors.contains(&a_id), "D's ancestors should include A");
        assert!(d_meta.ancestors.contains(&b_id), "D's ancestors should include B");
        assert!(d_meta.ancestors.contains(&c_id), "D's ancestors should include C");
        
        // Verify there are no clashing or shadowed properties since all properties have unique names
        assert!(d_meta.clashing_properties.is_empty(), "There should be no clashing properties");
        assert!(d_meta.shadowed_properties.is_empty(), "There should be no shadowed properties");
    }
    
    #[test]
    fn test_diamond_inheritance_without_clash() {
        // Test diamond inheritance where a property from the common ancestor is inherited through multiple paths
        // A (common_prop) -> B -> D
        //                \-> C -/
        // In this case, D inherits common_prop from A through both B and C, but there's no clash
        let mut registry = InMemoryRegistry::new();
        
        // Create class A with property "common_prop"
        let a_id = setup_class(
            &mut registry,
            "A",
            HashSet::new(),
            vec![("common_prop", Type::Int)],
        );
        
        // Create class B inheriting from A
        let b_id = setup_class(
            &mut registry,
            "B",
            HashSet::from([a_id]),
            vec![("b_prop", Type::Float)],
        );
        
        // Create class C inheriting from A
        let c_id = setup_class(
            &mut registry,
            "C",
            HashSet::from([a_id]),
            vec![("c_prop", Type::String)],
        );
        
        // Create class D inheriting from both B and C
        let d_id = setup_class(
            &mut registry,
            "D",
            HashSet::from([b_id, c_id]),
            vec![("d_prop", Type::Int)],
        );
        
        // Verify D's accessible properties
        let d_meta = registry.get_class(d_id).unwrap();
        
        // D should have access to common_prop from A
        assert!(d_meta.accessble_properties.contains_key("common_prop"), "D should inherit common_prop from A");
        
        // The source of common_prop should still be A
        let common_prop = d_meta.accessble_properties.get("common_prop").unwrap();
        assert_eq!(common_prop.source, a_id, "The source of common_prop should be A");
        
        // Verify there are no clashing properties
        assert!(d_meta.clashing_properties.is_empty(), "There should be no clashing properties");
    }
    
    #[test]
    fn test_property_clashing() {
        // Test property clashing when inheriting properties with the same name from different sources
        let mut registry = InMemoryRegistry::new();
        
        // Create class X with property "shared_name"
        let x_id = setup_class(
            &mut registry,
            "X",
            HashSet::new(),
            vec![("shared_name", Type::Int)],
        );
        
        // Create class Y with property "shared_name" (different source, same name)
        let y_id = setup_class(
            &mut registry,
            "Y",
            HashSet::new(),
            vec![("shared_name", Type::Float)], // Note: Different type
        );
        
        // Create class Z inheriting from both X and Y
        let z_id = setup_class(
            &mut registry,
            "Z",
            HashSet::from([x_id, y_id]),
            vec![("z_prop", Type::String)],
        );
        
        // Verify Z's properties
        let z_meta = registry.get_class(z_id).unwrap();
        
        // Z should have a clashing property "shared_name"
        assert!(z_meta.clashing_properties.contains_key("shared_name"), "Z should have clashing shared_name");
        
        // The clashing set should contain properties from both X and Y
        let clashing = z_meta.clashing_properties.get("shared_name").unwrap();
        assert_eq!(clashing.len(), 2, "There should be two clashing properties");
        
        // Verify one property is from X and one is from Y
        let sources: HashSet<ClassID> = clashing.iter().map(|p| p.source).collect();
        assert!(sources.contains(&x_id), "One clashing property should be from X");
        assert!(sources.contains(&y_id), "One clashing property should be from Y");
        
        // Verify Z's accessible properties don't contain "shared_name"
        assert!(!z_meta.accessble_properties.contains_key("shared_name"), 
               "Z should not have shared_name in accessible properties due to clash");
    }
    
    #[test]
    fn test_shadow_resolving_clash() {
        // Test case where a class defines a property that shadows clashing inherited properties
        let mut registry = InMemoryRegistry::new();
        
        // Create class X with property "shared_name"
        let x_id = setup_class(
            &mut registry,
            "X",
            HashSet::new(),
            vec![("shared_name", Type::Int)],
        );
        
        // Create class Y with property "shared_name" (different source, same name)
        let y_id = setup_class(
            &mut registry,
            "Y",
            HashSet::new(),
            vec![("shared_name", Type::Float)],
        );
        
        // Create class Z inheriting from both X and Y (will have clash)
        let z_id = setup_class(
            &mut registry,
            "Z",
            HashSet::from([x_id, y_id]),
            vec![],
        );
        
        // Verify Z has clashing property
        let z_meta = registry.get_class(z_id).unwrap();
        assert!(z_meta.clashing_properties.contains_key("shared_name"), 
               "Z should have clashing shared_name properties");
        
        // Create class W inheriting from Z but defining its own "shared_name" property
        let w_id = setup_class(
            &mut registry,
            "W",
            HashSet::from([z_id]),
            vec![("shared_name", Type::String)], // W defines its own shared_name
        );
        
        // Verify W's properties
        let w_meta = registry.get_class(w_id).unwrap();
        
        // W should not have clashing "shared_name" property anymore
        assert!(!w_meta.clashing_properties.contains_key("shared_name"), 
               "W should not have clashing shared_name due to shadowing");
        
        // W should have shadowed properties
        assert!(w_meta.shadowed_properties.contains_key("shared_name"), 
               "W should have shadowed shared_name properties");
        
        // The number of shadowed properties should be 2 (from X and Y)
        let shadowed = w_meta.shadowed_properties.get("shared_name").unwrap();
        assert_eq!(shadowed.len(), 2, "W should shadow 2 properties (from X and Y)");
        
        // The accessible property should be W's own
        let accessible_prop = w_meta.accessble_properties.get("shared_name").unwrap();
        assert_eq!(accessible_prop.source, w_id, "The accessible shared_name should be W's own");
    }
    
    #[test]
    fn test_multi_level_inheritance() {
        // Test multi-level inheritance (5 levels)
        // A -> B -> C -> D -> E
        let mut registry = InMemoryRegistry::new();
        
        // Create classes with properties at each level
        let a_id = setup_class(
            &mut registry,
            "A",
            HashSet::new(),
            vec![("a_prop", Type::Int), ("common", Type::Int)],
        );
        
        let b_id = setup_class(
            &mut registry,
            "B",
            HashSet::from([a_id]),
            vec![("b_prop", Type::Float)],
        );
        
        let c_id = setup_class(
            &mut registry,
            "C",
            HashSet::from([b_id]),
            vec![("c_prop", Type::String), ("common", Type::String)], // Shadows A's common
        );
        
        let d_id = setup_class(
            &mut registry,
            "D",
            HashSet::from([c_id]),
            vec![("d_prop", Type::Int)],
        );
        
        let e_id = setup_class(
            &mut registry,
            "E",
            HashSet::from([d_id]),
            vec![("e_prop", Type::Float)],
        );
        
        // Verify E's properties
        let e_meta = registry.get_class(e_id).unwrap();
        
        // E should have access to properties from all levels
        assert!(e_meta.accessble_properties.contains_key("a_prop"), "E should inherit a_prop from A");
        assert!(e_meta.accessble_properties.contains_key("b_prop"), "E should inherit b_prop from B");
        assert!(e_meta.accessble_properties.contains_key("c_prop"), "E should inherit c_prop from C");
        assert!(e_meta.accessble_properties.contains_key("d_prop"), "E should inherit d_prop from D");
        assert!(e_meta.accessble_properties.contains_key("e_prop"), "E should have its own e_prop");
        
        // E should have access to "common" from C (not A)
        assert!(e_meta.accessble_properties.contains_key("common"), "E should inherit common");
        let common_prop = e_meta.accessble_properties.get("common").unwrap();
        assert_eq!(common_prop.source, c_id, "The source of common should be C (not A)");
        
        // A's "common" property should be shadowed by C's
        assert!(e_meta.shadowed_properties.contains_key("common"), "A's common should be shadowed");
        let shadowed = e_meta.shadowed_properties.get("common").unwrap();
        assert_eq!(shadowed.len(), 1, "One property should be shadowed");
        let shadowed_prop = shadowed.iter().next().unwrap();
        assert_eq!(shadowed_prop.source, a_id, "Shadowed property should be from A");
        
        // Verify ancestors
        assert!(e_meta.ancestors.contains(&a_id), "E's ancestors should include A");
        assert!(e_meta.ancestors.contains(&b_id), "E's ancestors should include B");
        assert!(e_meta.ancestors.contains(&c_id), "E's ancestors should include C");
        assert!(e_meta.ancestors.contains(&d_id), "E's ancestors should include D");
    }
    
    #[test]
    fn test_complex_diamond_with_shadowing_and_clashing() {
        // Test complex diamond with shadowing and clashing
        //     A (prop1)
        //    / \
        //   B   C (prop1, prop2)
        //  / \ /
        // D   E (prop2)
        //  \ /
        //   F (prop3)
        let mut registry = InMemoryRegistry::new();
        
        // Create classes
        let a_id = setup_class(
            &mut registry,
            "A",
            HashSet::new(),
            vec![("prop1", Type::Int)],
        );
        
        let b_id = setup_class(
            &mut registry,
            "B",
            HashSet::from([a_id]),
            vec![],
        );
        
        let c_id = setup_class(
            &mut registry,
            "C",
            HashSet::from([a_id]),
            vec![("prop1", Type::Float), ("prop2", Type::String)], // C shadows A's prop1
        );
        
        let d_id = setup_class(
            &mut registry,
            "D",
            HashSet::from([b_id]),
            vec![],
        );
        
        let e_id = setup_class(
            &mut registry,
            "E",
            HashSet::from([b_id, c_id]),
            vec![("prop2", Type::Int)], // E shadows C's prop2
        );
        
        let f_id = setup_class(
            &mut registry,
            "F",
            HashSet::from([d_id, e_id]),
            vec![("prop3", Type::Float)],
        );
        
        // Verify F's properties
        let f_meta = registry.get_class(f_id).unwrap();
        
        // F should inherit prop1 from somewhere, but there's potential for clash
        // When E inherits from B and C, there are two prop1 sources: A (via B) and C
        // Check that F has clashing prop1 properties
        assert!(f_meta.clashing_properties.contains_key("prop1"), 
               "F should have clashing prop1 properties from A and C");
        
        let prop1_clash = f_meta.clashing_properties.get("prop1").unwrap();
        assert_eq!(prop1_clash.len(), 2, "Should be two clashing prop1 properties");
        
        // F should inherit prop2 from E
        assert!(f_meta.accessble_properties.contains_key("prop2"), "F should inherit prop2 from E");
        let prop2 = f_meta.accessble_properties.get("prop2").unwrap();
        assert_eq!(prop2.source, e_id, "prop2 should be from E");
        
        // C's prop2 should be shadowed in F (via E's shadowing)
        assert!(f_meta.shadowed_properties.contains_key("prop2"), 
               "C's prop2 should be shadowed in F");
        
        // F should have its own prop3
        assert!(f_meta.accessble_properties.contains_key("prop3"), "F should have its own prop3");
    }
}