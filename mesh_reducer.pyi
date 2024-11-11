from pathlib import Path
from typing import List, Tuple

def reduce_mesh_points(
    file_path: Path, clusters: int
) -> List[Tuple[float, float, float]]:
    """Reduces the number of points in a mesh file using fast grid clustering.

    Args:
        file_path (Path): Path to the mesh file.
        clusters (int): Number of clusters to reduce the points to.

    Returns:
        List[Tuple[float, float, float]]: A list of tuples containing the reduced points.
    """

def reduce_mesh_points_multi(
    file_paths: List[Path], clusters: int
) -> List[List[Tuple[float, float, float]]]:
    """Reduces the number of points in multiple mesh files using fast grid clustering.

    Args:
        file_paths (List[Path]): List of paths to the mesh files.
        clusters (int): Number of clusters to reduce the points to.

    Returns:
        List[List[Tuple[float, float, float]]]: A list of lists of tuples containing the reduced points for each file.
    """
