# protodefc
Tool which lets you write a specification of a binary format as a PDS (protodef spec) file. The PDS file can then be compiled into a number of languages.

Documentation for the binary format can also be generated.

Supported languages (in order of priority):

* [x] Javascript
* [ ] Java
* [x] Python
* [ ] Rust

More languages are planned.

## Example
```ruby
@doc """
This is a documentation attribute. It will be included in the 
generated documentation.

A native type is a type which needs to be implemented in each 
target language.

You would need to provide a function for reading, writing and 
calculating the size of the type.

The function is annotated as an integer so that the compiler 
can use it in things like array length calculations.
"""
@type integer("u8") def_native("u8");
@type integer("u64") def_native("u64");

@doc "Native types without type annotations are opaque to the compiler."
def_native("f64");

@doc "A native type which takes a single integer argument."
@type binary("utf8")
def_native("sized_string") {
    @doc """
    Arguments can also have documentation attached to them. This
    will show up in the documentation browser.
    """
    argument("size", stage: "read") => integer("usize");
};

@doc "A composite data structure which contains 3 fields."
def("position") => container {
    field("x") => ::f64;
    field("y") => ::f64;
    field("z") => ::f64;
};

@doc """
A more advanced composite data structure.

Demonstrates usage of virtual fields.
"""
def("entity_data") => container {
    field("entity_id") => ::u64;
    
    @doc "Uses the type defined above."
    field("position") => ::position;
    
    @doc """
    A virtual field will exist only in the binary form, not in the
    input/output data structure. A virtual field always needs another
    field to pull its value from when serializing. In this case it
    writes the tag of the `entity_type` union field.
    """
    virtual_field("entity_type_tag", value: "entity_type/@tag") => ::u8;
    
    @doc """
    A union is the only provided way to do conditionals.
    
    This will match on the value of the `entity_type_tag` field in the
    parent container.
    """
    field("entity_type") => union("entity_type", tag: "../entity_type_tag") {
        variant("player", match: "0") => container {};
        variant("zombie", match: "1") => container {};
    };
};

@doc """
A composite type which reads and writes a u8 size prefixed utf8 
string.

Note that since the container is `virtual`, it will not actually 
exist in the input/output data structure. 
This will look like a utf8 string to the programmer.
"""
def("u8_string") => container(virtual: "true") {
    virtual_field("size", value: "string/@size") => u8;
    field("string") => sized_string(size: "../size");
};

@doc "Another example of a virtual container."
def("packet") => container(virtual: "true") {
    virtual_field("tag", value: "data/@tag") => ::u8;
    field("data") => union("packet_variant", tag: "../tag") {
        variant("handshake", match: "0") => container {
            field("nonce") => ::u64;
        };
        variant("ping", match: "1") => container {
            field("nonce") => ::u64;
            field("timestamp") => ::u64;
        };
        variant("spawn_entity", match: "2") => ::entity_data;
    };
};

@doc """
Namespaces are fully supported. They can be nested, and can
have documentation attached.
"""
namespace("test") {

  def("test") => container {};

};
```
