module ElmUser exposing (..)

type alias ElmUser =
	{ id: List(Dict String(List User))
	, vector: List Int
	, foo: Maybe(List Int)
	}
