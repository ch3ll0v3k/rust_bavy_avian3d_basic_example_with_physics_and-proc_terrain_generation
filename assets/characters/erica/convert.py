import aspose.threed as a3d

scene = a3d.Scene.from_file("Erika-Archer-With-Bow-Arrow.fbx")
scene.save("Erika-Archer-With-Bow-Arrow.fbx.glb")
