from pathlib import Path
from typing import List, Tuple

class MeshPoints:
    """Class to store the name of the mesh and its points."""

    name: str
    points: List[Tuple[float, float, float]]

def load_mesh(file_path: Path, clusters: int) -> MeshPoints:
    """Reduces the number of points in a mesh file using fast grid clustering.

    Args:
        file_path (Path): Path to the mesh file.
        clusters (int): Number of clusters to reduce the points to.

    Returns:
        MeshPoints: A MeshPoints object with reduced points.
    """

def load_meshes(file_paths: List[Path], clusters: int) -> List[MeshPoints]:
    """Reduces the number of points in multiple mesh files using fast grid clustering.

    Args:
        file_paths (List[Path]): List of paths to the mesh files.
        clusters (int): Number of clusters to reduce the points to.

    Returns:
        List[MeshPoints]: A list of MeshPoints objects with reduced points for each mesh file.
    """

def load_meshes_range_points(
    file_paths: List[Path], clusters_range: List[int]
) -> List[MeshPoints]:
    """Reduces the number of points in multiple mesh files using fast grid clustering with a range of clusters.

    Args:
        file_paths (List[Path]): List of paths to the mesh files.
        clusters_range (List[int]): List of cluster values to reduce the points to.

    Returns:
        List[MeshPoints]: A list of MeshPoints objects with reduced points for each mesh file.
    """
