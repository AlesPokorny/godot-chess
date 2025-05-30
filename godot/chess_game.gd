extends Node2D


func _ready():
	var game = GodotGame.new();
	add_child(game);
	if Globals.fen_string == "":
		game.start(Globals.player_color)
	else:
		game.start_from_fen(Globals.fen_string)
