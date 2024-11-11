import pyvista as pv
from mesh_reducer import reduce_mesh_points_multi
from pathlib import Path
from time import perf_counter

paths = list(Path("tests/assets").rglob("*.stl"))
t = perf_counter()
res = reduce_mesh_points_multi(paths, 10_000)
print(f"Time taken: {perf_counter() - t:.2f}s")

for i, r in enumerate(res):
    print(f"Reduced points for {paths[i]}: {len(r)}")
    try:
        cloud = pv.PolyData(r)
        plotter = pv.Plotter()
        plotter.add_mesh(cloud, point_size=30, render_points_as_spheres=True)
        plotter.add_title("")
        plotter.show()
    except Exception as e:
        print(f"Ошибка визуализации: {e}")
