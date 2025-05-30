extends Node2D

@onready var text_input := get_node("CenterContainer/VBoxContainer/FenInput")


func _on_start_button_pressed() -> void:
	var fen_string = text_input.get_text()
	var game = GodotGame.new()
	game.check_fen_string(fen_string)
	
	if game.check_fen_string(fen_string):
		game.free()
		Globals.fen_string = fen_string
		get_tree().change_scene_to_file("res://chess_game.tscn")


func _on_back_button_2_pressed() -> void:
	get_tree().change_scene_to_file("res://menu/menu.tscn")
