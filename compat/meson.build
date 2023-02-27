compat_library_native = static_library('compat_native', 'strlcat.c', 'strlcpy.c', include_directories: [ common_includes ], native: true)
compat_native_dependency = declare_dependency(link_with : compat_library_native)

compat_library = static_library('compat', 'strlcat.c', 'strlcpy.c', include_directories: [ common_includes ], native: false)
compat_dependency = declare_dependency(link_with : compat_library)