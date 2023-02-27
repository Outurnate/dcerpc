uuid = static_library('uuid',
  'uuid.c',
  'uuidsys.c',
  'get_802_addr.c',
  include_directories : [ common_includes ]
)

uuid_dependency = declare_dependency(
  link_with : uuid,
  include_directories : include_directories('.')
)