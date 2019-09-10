module User exposing (..)

type alias User =
	{ id: List(Dict String(List String))
	, elm_rename: Maybe(List Int)
	, vector: List Int
	}
