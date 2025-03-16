use std::collections::{HashSet,HashMap};


//we assume 64bit word size
pub type ClassID = u32;
#[repr(u32)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Type{
    Int=0,
    Float=1,
    String=2,
    Class(ClassID)
}

impl From<Type> for u64 {
	#[inline(always)]
    fn from(t: Type) -> Self {
        match t {
            Type::Int => 0u64,
            Type::Float => 1u64,
            Type::String => 2u64,
            // Shift the ClassID up by 32 bits to move it completely out of the discriminant range
            Type::Class(id) => 3u64 | ((id as u64) << 32)
        }
    }
}

impl From<Type> for usize {
    fn from(t: Type) -> Self {
        u64::from(t) as usize
    }
}


#[cfg(test)]
mod tests {
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

pub struct Property{
	pub id: Type,
	pub source: ClassID,
}

pub struct ClassMeta{
	pub parents: HashSet<ClassID>,
	pub properties: HashMap<String,Property>,
	pub clashing_properties: HashMap<String,Vec<Property>>,
}