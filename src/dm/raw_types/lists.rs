use super::values;

/*struct RawList
{
	Value* vector_part;
	AssociativeListEntry* map_part;
	int allocated_size; //maybe
	int length;
	int refcount;
	int unk3; //this one appears to be a pointer to a struct holding the vector_part pointer, a zero, and maybe the initial size? no clue.

	bool is_assoc()
	{
		return map_part != nullptr;
	}

};

enum class RbtColor : bool
{
	Black = false,
	Red = true,
};

struct AssociativeListEntry
{
	Value key;
	Value value;
	RbtColor color;
	AssociativeListEntry* left;
	AssociativeListEntry* right;
};
*/

#[repr(C)]
enum Color {
	Red = 0,
	Black = 1,
}

#[repr(C)]
struct AssociativeListEntry {
	key: values::Value,
	value: values::Value,
	color: Color,
	left: *mut AssociativeListEntry,
	right: *mut AssociativeListEntry,
}

#[repr(C)]
pub struct List {
	vector_part: *mut values::Value,
	assoc_part: *mut AssociativeListEntry,
	allocated: u32,
	length: u32,
	refcount: u32,
	unknown: u32,
}
