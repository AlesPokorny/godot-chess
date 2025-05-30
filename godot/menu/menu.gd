extends Node

func _start_game():
	get_tree().change_scene_to_file("res://chess_game.tscn")

func _on_start_button_white_pressed() -> void:
	Globals.player_color = "white";
	_start_game()

func _on_start_button_black_pressed() -> void:
	Globals.player_color = "black";
	_start_game()

func _on_start_button_fen_pressed() -> void:
	get_tree().change_scene_to_file("res://menu/from_fen.tscn")

func _on_quit_button_pressed() -> void:
	get_tree().quit(0)

func _on_options_button_pressed() -> void:
	get_tree().change_scene_to_file("res://menu/options.tscn")
