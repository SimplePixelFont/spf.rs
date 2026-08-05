# `spf.rs` as a library
This article discusses `spf.rs` usage as a library in C. However, the same principles can be applied to any language that has a Foreign Function Interface (FFI) which adhere to the platform specific C-ABI. This includes programming languages such as Python, Julia, Ruby, Java, WASM, C/C++, etc.

First obtain a copy of the `spf.rs` library binary by either downloading from the [releases](https://github.com/SimplePixelFont/spf.rs/releases) section, or [compiling spf.rs from source](https://docs.rs/spf/latest/spf/articles/installing/index.html#compiling-spfrs-from-source).

### Header Files
Additionally, to use `spf.rs` in C/C++, download `spf.h` found in the [releases](https://github.com/SimplePixelFont/spf.rs/releases) section. Then add the following header in your C code:
```c
#include "spf.h"
```

### Loading the Library
The first step is to load the `spf.rs` library. On Linux and macOS this is the `dlopen()` function from the POSIX `dlfcn.h` header; Windows has no `dlfcn.h` and uses `LoadLibraryA()` from `windows.h` instead. Wrapping both behind a small set of macros keeps the rest of the code platform-independent:
```c
#if defined(_WIN32)
    #include <windows.h>
    #define SPF_LIB_HANDLE HMODULE
    #define SPF_LIB_OPEN(path) LoadLibraryA(path)
    #define SPF_LIB_SYM(handle, name) GetProcAddress(handle, name)
    #define SPF_LIB_CLOSE(handle) FreeLibrary(handle)
    #define SPF_LIBRARY_FILE "spf.dll"
    static const char *spf_lib_last_error(void) {
        static char message[256];
        FormatMessageA(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            NULL, GetLastError(), 0, message, sizeof(message), NULL
        );
        return message;
    }
    #define SPF_LIB_LAST_ERROR() spf_lib_last_error()
#else
    #include <dlfcn.h>
    #define SPF_LIB_HANDLE void*
    #define SPF_LIB_OPEN(path) dlopen(path, RTLD_LAZY)
    #define SPF_LIB_SYM(handle, name) dlsym(handle, name)
    #define SPF_LIB_CLOSE(handle) dlclose(handle)
    #define SPF_LIB_LAST_ERROR() dlerror()
    // Linux and macOS share the same dlopen/dlsym API and only the file extension differs.
    #if defined(__APPLE__)
        #define SPF_LIBRARY_FILE "libspf.dylib"
    #else
        #define SPF_LIBRARY_FILE "libspf.so"
    #endif
#endif
```
With that in place, loading the library looks the same on every platform:
```c
printf("Loading " SPF_LIBRARY_FILE "\n");

SPF_LIB_HANDLE handle = SPF_LIB_OPEN("./" SPF_LIBRARY_FILE);
if (!handle) {
    printf("%s\n", SPF_LIB_LAST_ERROR());
    return 1;
}

printf("Loading " SPF_LIBRARY_FILE " succeeded\n");
```

### Define symbols
Next we need to store the function symbols from the library into variables so we can use them in our program. Note that `spf_core_layout_from_data` and `spf_core_layout_to_data` take an out-parameter and return an `SPFStatus` rather than returning the struct directly in a result like the Rust API.
```c
SPFStatus (*spf_core_layout_from_data)(const unsigned char*, unsigned long, struct SPFLayout*);
SPFStatus (*spf_core_layout_to_data)(const struct SPFLayout*, struct SPFData*);
void (*spf_free_layout)(struct SPFLayout);
void (*spf_free_data)(struct SPFData);

// We can assign the variables as follows
spf_core_layout_from_data = (SPFStatus (*)(const unsigned char*, unsigned long, struct SPFLayout*))
    SPF_LIB_SYM(handle, "spf_core_layout_from_data");
spf_core_layout_to_data = (SPFStatus (*)(const struct SPFLayout*, struct SPFData*))
    SPF_LIB_SYM(handle, "spf_core_layout_to_data");
spf_free_layout = (void (*)(struct SPFLayout))SPF_LIB_SYM(handle, "spf_free_layout");
spf_free_data = (void (*)(struct SPFData))SPF_LIB_SYM(handle, "spf_free_data");
```
### Extra
We can now use the symbols we defined and begin calling `spf.rs` functions. However for our example, here is also a function that loads a file into a buffer in C. We will use this in the next step:
```c
int read_file_to_buffer(char **buffer, unsigned long *file_size) {
    FILE *file;

    file = fopen("path/to/font.spf", "rb");
    if (file == NULL) {
        printf("Error opening file\n");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    *file_size = ftell(file);
    rewind(file);

    *buffer = (char*)malloc(*file_size);
    if (*buffer == NULL) {
        printf("Memory allocation failed\n");
        fclose(file);
        return 1;
    }

    size_t bytes_read = fread(*buffer, 1, *file_size, file);

    if (bytes_read != (size_t)*file_size) {
        printf("Error reading file\n");
        free(*buffer);
        fclose(file);
        return 1;
    }

    fclose(file);

    return 0;
}
```
### Calling `spf.rs` functions
Now that we have our symbols defined, here is a simple script that uses the above function to load a `spf.rs` file and extract all the fields / characters. Since `spf_core_layout_from_data` returns an `SPFStatus`, always check it's `Ok` before reading the out-parameter:
```c
struct SPFLayout layout;
SPFStatus status = spf_core_layout_from_data((unsigned char*)buffer, file_size, &layout);
if (status != Ok) {
    printf("spf_core_layout_from_data failed with status %d\n", status);
    return 1;
}

printf("---Header Data---\n");
printf("Format Version: %d\n", layout.version);
printf("Compact: %s\n", layout.compact ? "true" : "false");

printf("---Character Tables---\n");
for (unsigned long i = 0; i < layout.character_tables_length; i++) {
    struct SPFCharacterTable *table = &layout.character_tables[i];
    printf("Character Table %lu:\n", i);
    printf("  Use advance_x: %s\n",
        (table->modifier_flags & SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_ADVANCE_X) ? "true" : "false");
    printf("  Use pixmap_index: %s\n",
        (table->modifier_flags & SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_PIXMAP_INDEX) ? "true" : "false");
    printf("  Constant Code Point Count: %s(%d)\n",
        table->has_constant_code_point_count ? "true" : "false",
        table->constant_code_point_count
    );
    printf("  Pixmap Table Indexes: %s(", table->has_pixmap_table_indexes ? "true" : "false");
    for (unsigned long j = 0; j < table->pixmap_table_indexes_length; j++) {
        printf(j == table->pixmap_table_indexes_length - 1 ? "%d" : "%d, ", table->pixmap_table_indexes[j]);
    }
    printf(")\n");
    printf("  Characters:\n");
    for (unsigned long j = 0; j < table->characters_length; j++) {
        struct SPFCharacter *character = &table->characters[j];
        printf("  - Character %lu:\n", j);
        printf("    advance_x: %s(%d)\n", character->has_advance_x ? "true" : "false", character->advance_x);
        printf("    pixmap_index: %s(%d)\n", character->has_pixmap_index ? "true" : "false", character->pixmap_index);
        printf("    code_points: '%s'\n", character->code_points);
    }
}

printf("---Color Tables---\n");
for (unsigned long i = 0; i < layout.color_tables_length; i++) {
    struct SPFColorTable *table = &layout.color_tables[i];
    printf("Color Table %lu:\n", i);
    printf("  Constant Alpha: %s(%d)\n", table->has_constant_alpha ? "true" : "false", table->constant_alpha);
    printf("  Colors:\n");
    for (unsigned long j = 0; j < table->colors_length; j++) {
        struct SPFColor *color = &table->colors[j];
        printf("  - Color %lu:\n", j);
        printf("    custom_alpha: %s(%d)\n", color->has_custom_alpha ? "true" : "false", color->custom_alpha);
        printf("    red: %d\n", color->red);
        printf("    green: %d\n", color->green);
        printf("    blue: %d\n", color->blue);
    }
}

printf("---Pixmap Tables---\n");
for (unsigned long i = 0; i < layout.pixmap_tables_length; i++) {
    struct SPFPixmapTable *table = &layout.pixmap_tables[i];
    printf("Pixmap Table %lu:\n", i);
    printf("  Constant Width: %s(%d)\n", table->has_constant_width ? "true" : "false", table->constant_width);
    printf("  Constant Height: %s(%d)\n", table->has_constant_height ? "true" : "false", table->constant_height);
    printf("  Constant Bits Per Pixel: %s(%d)\n",
        table->has_constant_bits_per_pixel ? "true" : "false", table->constant_bits_per_pixel);
    printf("  Color Table Indexes: %s(", table->has_color_table_indexes ? "true" : "false");
    for (unsigned long j = 0; j < table->color_table_indexes_length; j++) {
        printf(j == table->color_table_indexes_length - 1 ? "%d" : "%d, ", table->color_table_indexes[j]);
    }
    printf(")\n");
    printf("  Pixmaps:\n");
    for (unsigned long j = 0; j < table->pixmaps_length; j++) {
        struct SPFPixmap *pixmap = &table->pixmaps[j];
        printf("  - Pixmap %lu:\n", j);
        printf("    custom_width: %s(%d)\n", pixmap->has_custom_width ? "true" : "false", pixmap->custom_width);
        printf("    custom_height: %s(%d)\n", pixmap->has_custom_height ? "true" : "false", pixmap->custom_height);
        printf("    custom_bits_per_pixel: %s(%d)\n",
            pixmap->has_custom_bits_per_pixel ? "true" : "false", pixmap->custom_bits_per_pixel);
        printf("    data: ");
        for (unsigned long k = 0; k < pixmap->data_length; k++) {
            printf("%d ", pixmap->data[k]);
        }
        printf("\n");
    }
}

printf("---Font Tables---\n");
for (unsigned long i = 0; i < layout.font_tables_length; i++) {
    struct SPFFontTable *table = &layout.font_tables[i];
    printf("Font Table %lu:\n", i);
    printf("  Character Table Indexes: %s(", table->has_character_table_indexes ? "true" : "false");
    for (unsigned long j = 0; j < table->character_table_indexes_length; j++) {
        printf(j == table->character_table_indexes_length - 1 ? "%d" : "%d, ", table->character_table_indexes[j]);
    }
    printf(")\n");
    printf("  Fonts:\n");
    for (unsigned long j = 0; j < table->fonts_length; j++) {
        struct SPFFont *font = &table->fonts[j];
        printf("  - Font %lu:\n", j);
        printf("    name: '%s'\n", font->name);
        printf("    author: '%s'\n", font->author);
        printf("    version: %d\n", font->version);
        printf("    font_type (bit flags): %d\n", font->font_type);
        printf("    linked_character_table_indexes: (");
        for (unsigned long k = 0; k < font->linked_character_table_indexes_length; k++) {
            printf(k == font->linked_character_table_indexes_length - 1 ? "%d" : "%d, ",
                font->linked_character_table_indexes[k]);
        }
        printf(")\n");
    }
}
```
Running this against [`res/sampleToyFont.spf`](https://github.com/SimplePixelFont/spf.rs/blob/main/res/sampleToyFont.spf), the same file the Rust integration tests use, prints:
```text
---Header Data---
Format Version: 0
Compact: true
---Character Tables---
Character Table 0:
  Use advance_x: false
  Use pixmap_index: false
  Constant Code Point Count: false(0)
  Pixmap Table Indexes: true(0)
  Characters:
  - Character 0:
    advance_x: false(0)
    pixmap_index: false(0)
    code_points: 'o'
...
---Font Tables---
Font Table 0:
  Character Table Indexes: true(0)
  Fonts:
  - Font 0:
    name: 'SampleToyFont'
    author: 'The-Nice-One'
    version: 0
    font_type (bit flags): 0
    linked_character_table_indexes: (0)
```
And now that we have a `crate::core::Layout` in C, or more precisely a `core::ffi::SPFLayout`, we can also convert it back into data. `spf_core_layout_to_data` follows the same out-parameter/`SPFStatus` pattern:
```c
struct SPFData data;
status = spf_core_layout_to_data(&layout, &data);
if (status != Ok) {
    printf("spf_core_layout_to_data failed with status %d\n", status);
    return 1;
}

printf("Data: ");
for (unsigned long i = 0; i < data.data_length; i++) {
    printf("%d ", data.data[i]);
}
printf("\n");
```
Finally, free everything `spf.rs` allocated with `spf_free_data` for the `SPFData`, `spf_free_layout` for the `SPFLayout`, and `SPF_LIB_CLOSE` for the library handle itself:
```c
spf_free_data(data);
spf_free_layout(layout);
SPF_LIB_CLOSE(handle);
```

### Full code for Copy & Paste
Minimal Changes: Filepaths are now defined as constants for even more Copy & Paste friendliness.
```c
#include <stdio.h>
#include <stdlib.h>
#include "spf.h"

#if defined(_WIN32)
    #include <windows.h>
    #define SPF_LIB_HANDLE HMODULE
    #define SPF_LIB_OPEN(path) LoadLibraryA(path)
    #define SPF_LIB_SYM(handle, name) GetProcAddress(handle, name)
    #define SPF_LIB_CLOSE(handle) FreeLibrary(handle)
    #define SPF_LIBRARY_FILE "spf.dll"
    static const char *spf_lib_last_error(void) {
        static char message[256];
        FormatMessageA(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            NULL, GetLastError(), 0, message, sizeof(message), NULL
        );
        return message;
    }
    #define SPF_LIB_LAST_ERROR() spf_lib_last_error()
#else
    #include <dlfcn.h>
    #define SPF_LIB_HANDLE void*
    #define SPF_LIB_OPEN(path) dlopen(path, RTLD_LAZY)
    #define SPF_LIB_SYM(handle, name) dlsym(handle, name)
    #define SPF_LIB_CLOSE(handle) dlclose(handle)
    #define SPF_LIB_LAST_ERROR() dlerror()
    #if defined(__APPLE__)
        #define SPF_LIBRARY_FILE "libspf.dylib"
    #else
        #define SPF_LIBRARY_FILE "libspf.so"
    #endif
#endif

#define FILENAME "./sampleToyFont.spf"
#define SPF_LIBRARY "./" SPF_LIBRARY_FILE

int read_file_to_buffer(char **buffer, unsigned long *file_size) {
    FILE *file;

    file = fopen(FILENAME, "rb");
    if (file == NULL) {
        printf("Error opening file\n");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    *file_size = ftell(file);
    rewind(file);

    *buffer = (char*)malloc(*file_size);
    if (*buffer == NULL) {
        printf("Memory allocation failed\n");
        fclose(file);
        return 1;
    }

    size_t bytes_read = fread(*buffer, 1, *file_size, file);
    if (bytes_read != (size_t)*file_size) {
        printf("Error reading file\n");
        free(*buffer);
        fclose(file);
        return 1;
    }

    fclose(file);
    return 0;
}

int main() {
    char *buffer;
    unsigned long file_size;

    int result = read_file_to_buffer(&buffer, &file_size);
    if (result != 0) {
        return result;
    }

    printf("Loading " SPF_LIBRARY_FILE "\n");

    SPF_LIB_HANDLE handle = SPF_LIB_OPEN(SPF_LIBRARY);
    if (!handle) {
        printf("%s\n", SPF_LIB_LAST_ERROR());
        return 1;
    }

    printf("Loading " SPF_LIBRARY_FILE " succeeded\n");

    SPFStatus (*spf_core_layout_from_data)(const unsigned char*, unsigned long, struct SPFLayout*);
    SPFStatus (*spf_core_layout_to_data)(const struct SPFLayout*, struct SPFData*);
    void (*spf_free_layout)(struct SPFLayout);
    void (*spf_free_data)(struct SPFData);

    spf_core_layout_from_data = (SPFStatus (*)(const unsigned char*, unsigned long, struct SPFLayout*))
        SPF_LIB_SYM(handle, "spf_core_layout_from_data");
    spf_core_layout_to_data = (SPFStatus (*)(const struct SPFLayout*, struct SPFData*))
        SPF_LIB_SYM(handle, "spf_core_layout_to_data");
    spf_free_layout = (void (*)(struct SPFLayout))SPF_LIB_SYM(handle, "spf_free_layout");
    spf_free_data = (void (*)(struct SPFData))SPF_LIB_SYM(handle, "spf_free_data");

    /* We can use spf.rs functions now that we have loaded and assigned them to variables */

    struct SPFLayout layout;
    SPFStatus status = spf_core_layout_from_data((unsigned char*)buffer, file_size, &layout);
    if (status != Ok) {
        printf("spf_core_layout_from_data failed with status %d\n", status);
        free(buffer);
        SPF_LIB_CLOSE(handle);
        return 1;
    }
    free(buffer);

    printf("---Header Data---\n");
    printf("Format Version: %d\n", layout.version);
    printf("Compact: %s\n", layout.compact ? "true" : "false");

    printf("---Character Tables---\n");
    for (unsigned long i = 0; i < layout.character_tables_length; i++) {
        struct SPFCharacterTable *table = &layout.character_tables[i];
        printf("Character Table %lu:\n", i);
        printf("  Use advance_x: %s\n",
            (table->modifier_flags & SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_ADVANCE_X) ? "true" : "false");
        printf("  Use pixmap_index: %s\n",
            (table->modifier_flags & SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_PIXMAP_INDEX) ? "true" : "false");
        printf("  Constant Code Point Count: %s(%d)\n",
            table->has_constant_code_point_count ? "true" : "false",
            table->constant_code_point_count
        );
        printf("  Pixmap Table Indexes: %s(", table->has_pixmap_table_indexes ? "true" : "false");
        for (unsigned long j = 0; j < table->pixmap_table_indexes_length; j++) {
            printf(j == table->pixmap_table_indexes_length - 1 ? "%d" : "%d, ", table->pixmap_table_indexes[j]);
        }
        printf(")\n");
        printf("  Characters:\n");
        for (unsigned long j = 0; j < table->characters_length; j++) {
            struct SPFCharacter *character = &table->characters[j];
            printf("  - Character %lu:\n", j);
            printf("    advance_x: %s(%d)\n", character->has_advance_x ? "true" : "false", character->advance_x);
            printf("    pixmap_index: %s(%d)\n", character->has_pixmap_index ? "true" : "false", character->pixmap_index);
            printf("    code_points: '%s'\n", character->code_points);
        }
    }

    printf("---Color Tables---\n");
    for (unsigned long i = 0; i < layout.color_tables_length; i++) {
        struct SPFColorTable *table = &layout.color_tables[i];
        printf("Color Table %lu:\n", i);
        printf("  Constant Alpha: %s(%d)\n", table->has_constant_alpha ? "true" : "false", table->constant_alpha);
        printf("  Colors:\n");
        for (unsigned long j = 0; j < table->colors_length; j++) {
            struct SPFColor *color = &table->colors[j];
            printf("  - Color %lu:\n", j);
            printf("    custom_alpha: %s(%d)\n", color->has_custom_alpha ? "true" : "false", color->custom_alpha);
            printf("    red: %d\n", color->red);
            printf("    green: %d\n", color->green);
            printf("    blue: %d\n", color->blue);
        }
    }

    printf("---Pixmap Tables---\n");
    for (unsigned long i = 0; i < layout.pixmap_tables_length; i++) {
        struct SPFPixmapTable *table = &layout.pixmap_tables[i];
        printf("Pixmap Table %lu:\n", i);
        printf("  Constant Width: %s(%d)\n", table->has_constant_width ? "true" : "false", table->constant_width);
        printf("  Constant Height: %s(%d)\n", table->has_constant_height ? "true" : "false", table->constant_height);
        printf("  Constant Bits Per Pixel: %s(%d)\n",
            table->has_constant_bits_per_pixel ? "true" : "false", table->constant_bits_per_pixel);
        printf("  Color Table Indexes: %s(", table->has_color_table_indexes ? "true" : "false");
        for (unsigned long j = 0; j < table->color_table_indexes_length; j++) {
            printf(j == table->color_table_indexes_length - 1 ? "%d" : "%d, ", table->color_table_indexes[j]);
        }
        printf(")\n");
        printf("  Pixmaps:\n");
        for (unsigned long j = 0; j < table->pixmaps_length; j++) {
            struct SPFPixmap *pixmap = &table->pixmaps[j];
            printf("  - Pixmap %lu:\n", j);
            printf("    custom_width: %s(%d)\n", pixmap->has_custom_width ? "true" : "false", pixmap->custom_width);
            printf("    custom_height: %s(%d)\n", pixmap->has_custom_height ? "true" : "false", pixmap->custom_height);
            printf("    custom_bits_per_pixel: %s(%d)\n",
                pixmap->has_custom_bits_per_pixel ? "true" : "false", pixmap->custom_bits_per_pixel);
            printf("    data: ");
            for (unsigned long k = 0; k < pixmap->data_length; k++) {
                printf("%d ", pixmap->data[k]);
            }
            printf("\n");
        }
    }

    printf("---Font Tables---\n");
    for (unsigned long i = 0; i < layout.font_tables_length; i++) {
        struct SPFFontTable *table = &layout.font_tables[i];
        printf("Font Table %lu:\n", i);
        printf("  Character Table Indexes: %s(", table->has_character_table_indexes ? "true" : "false");
        for (unsigned long j = 0; j < table->character_table_indexes_length; j++) {
            printf(j == table->character_table_indexes_length - 1 ? "%d" : "%d, ", table->character_table_indexes[j]);
        }
        printf(")\n");
        printf("  Fonts:\n");
        for (unsigned long j = 0; j < table->fonts_length; j++) {
            struct SPFFont *font = &table->fonts[j];
            printf("  - Font %lu:\n", j);
            printf("    name: '%s'\n", font->name);
            printf("    author: '%s'\n", font->author);
            printf("    version: %d\n", font->version);
            printf("    font_type (bit flags): %d\n", font->font_type);
            printf("    linked_character_table_indexes: (");
            for (unsigned long k = 0; k < font->linked_character_table_indexes_length; k++) {
                printf(k == font->linked_character_table_indexes_length - 1 ? "%d" : "%d, ",
                    font->linked_character_table_indexes[k]);
            }
            printf(")\n");
        }
    }

    struct SPFData data;
    status = spf_core_layout_to_data(&layout, &data);
    if (status != Ok) {
        printf("spf_core_layout_to_data failed with status %d\n", status);
        spf_free_layout(layout);
        SPF_LIB_CLOSE(handle);
        return 1;
    }

    printf("Data: ");
    for (unsigned long i = 0; i < data.data_length; i++) {
        printf("%d ", data.data[i]);
    }
    printf("\n");

    spf_free_data(data);
    spf_free_layout(layout);
    SPF_LIB_CLOSE(handle);

    return 0;
}
```
