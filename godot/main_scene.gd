extends WfcNode3D

@export var spawnable_scenes: Array[PackedScene] = []

func spawn_road(segment: RoadSegment):
	var road_instance: Node3D = spawnable_scenes.get(segment.model_index).instantiate()
	road_instance.position = Vector3(segment.x, 0, segment.z)
	road_instance.rotation_degrees = Vector3(0, -segment.rot, 0)
	add_child(road_instance)
 
func _ready() -> void:
	var output: Array[RoadSegment] = make_road_rules(5, 5)
	for seg in output:
		print(seg.rot)
		spawn_road(seg)
