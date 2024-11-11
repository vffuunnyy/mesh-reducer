# Mesh Reducer

This project provides a Rust implementation for reading and reducing points from various 3D mesh file formats (STL, OBJ, and STEP) using a fast grid sampling method. The functionality is exposed to Python using PyO3.

## Features

- Read STL, OBJ, and STEP files and extract points.
- Reduce the number of points using a fast grid sampling method.
- Python bindings for seamless integration with Python projects.

## Supported Formats
- STL
- OBJ
- STEP (initial implementation with future improvements expected)

## Usage

Example usage in Python: [click here](https://github.com/vffuunnyy/ai_hack).

### Python

```bash
pip install mesh-reducer
```

## Functions

#### `reduce_mesh_points(file_path: Path, clusters: int) -> List[Tuple[float, float, float]]`
Reads a mesh file (STL, OBJ, STEP) and reduces the number of points.

**Args:**

- file_path (Path): Path to the mesh file.
- clusters (int): Number of clusters to reduce the points to.

**Returns:**

`List[Tuple[float, float, float]]`: A list of tuples containing the reduced points.

#### `reduce_mesh_points_multi(file_paths: List[Path], clusters: int) -> List[List[Tuple[float, float, float]]]`
Reads multiple mesh files (STL, OBJ, STEP) and reduces the number of points in each file.

**Args:**

- file_paths (List[Path]): List of paths to the mesh files.
- clusters (int): Number of clusters to reduce the points to.

**Returns:**

`List[List[Tuple[float, float, float]]]`: A list of lists of tuples containing the reduced points for each file.

## License

This project is licensed under the MIT License.