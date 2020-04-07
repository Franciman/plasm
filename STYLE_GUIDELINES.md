# Style Guidelines

Part of the style guidelines are specified in the .editorconfig file.
Here we add other conventions for C++ programs:

- Use Java-style braces:
```cpp
if(x == y) { // <- braces are put here, not on a separate line
```

- Class names are in CamelCase:
```cpp
class ClassName {
```

- Class fields are prefixed with m_ and are written in snake_case:
```cpp
class ClassName {
    int m_first_field;
```

- Access specifiers in classes are put at the same indentation level of the enclosing class:
```cpp
class ClassName {
private:
}
```

- All functions (both member functions and standalone functions) are written in snake_case:
```cpp
class ClassName {
public:
    int method_name();
};

void other_method();
```
