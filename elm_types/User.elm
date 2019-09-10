module User exposing (..)

type alias User =
	{ vector: List Int
	, foo: Maybe(List Int)
	, id: List(Dict String(List User))
	}
