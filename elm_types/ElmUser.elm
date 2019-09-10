module ElmUser exposing (..)

type alias ElmUser =
	{ vector: List Int
	, elm_rename: Maybe(List Int)
	, id: List(Dict String(List String))
	}
