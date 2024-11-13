from mesh_reducer import load_mesh, load_meshes
from pathlib import Path
from timeit import timeit


def read_mesh(file_path: Path, points: int = 2**15) -> list:
    return load_mesh(file_path, points)


def read_meshes(file_paths: list[Path], points: int = 2**15) -> list:
    return load_meshes(file_paths, points)


stl = Path("assets/DrivAer_F_D_WM_WW_0001.stl")
obj = Path("assets/0bi0pwinho2rjedcqwx8a9tf6.obj")


def test_single():
    print("Single mesh")
    print(timeit(lambda: read_mesh(stl), number=100))  # 2**15 points --> 12.05
    print(timeit(lambda: read_mesh(obj), number=100))  # 2**15 points --> 02.60


def test_multiple():
    print("Multiple meshes")
    print(timeit(lambda: read_meshes([stl, obj]), number=100))  # 2**15 points --> 12.69


if __name__ == "__main__":
    test_single()
    print("--------------------")
    test_multiple()

# import pyvista as pv
# try:
#     cloud = pv.PolyData(res)
#     plotter = pv.Plotter()
#     plotter.add_mesh(cloud, point_size=5, render_points_as_spheres=True)
#     plotter.add_title("")
#     plotter.show()
# except Exception as e:
#     print(f"Ошибка визуализации: {e}")
