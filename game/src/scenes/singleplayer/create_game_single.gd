extends Control


func _on_back_pressed():
	get_tree().change_scene_to_file("res://src/menus/start/start_menu.tscn")