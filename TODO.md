# Todo list

## WIP

- Start back from the basics and draw a simple triangle

## Global

- Refactor file locations / names
- Create routine to register engine level systems
- Create log to file
- Add debug specific code
- Create an example folder with many testbeds and way to run them from cargo directly
- Move the "init" and "shutdowns" logs inside the structures methods
- Add better documentation everywhere
- Check / Fix update function not running when window not focused ?
- Update the routine to Register engine Resources
- Update the routine to Register engine Components
- Update AABB structure to allow BVH construction

## ECS

- Add tests for the ECS methods
- Add a scheduler to manager ecs_ptr
- Add comments for Queries and the new system
- Add tests for the generational indices list

## Graphics

- Store meshes and materials as resources instead of ObjFiles
- Create materials and load them in obj
- Implement Shaders
- Implement GraphicsPipeline
- Implement Camera
- Implement Buffers
- Implement Images
- Implement Scene

## Maths

- Add vectors construction from smaller vectors
- Replace duplicate code with macros
- Add other matrix types (i32, etc...)
- Add other matrix structures (2x3, 2x4, ...)
- Add other matrix vector operations
- Add other tests + better docs for matrices

## Vulkan

- Use bindless with a single use descriptor set
- Put the physical device rate strategy in the configuration
- Add other strategies depending on the config: AR / VR / Raytracing / Embedded app / Offline simulation ...
- Add other features (ex PhysicalDeviceExtendedDynamicStateFeaturesEXT)

## Renderer

- Add user defined post processing effects