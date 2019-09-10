module ElmUser exposing (..)

type alias ElmUser =
	{ foo: Maybe(List Int)
	, vector: List Int
	, id: List(Dict String(List User))
	}
