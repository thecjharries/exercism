module RnaTranscription exposing (toRNA)

import Dict

complement : Dict.Dict Char Char
complement = Dict.fromList
    [ ( 'G', 'C' )
    , ( 'C', 'G' )
    , ( 'T', 'A' )
    , ( 'A', 'U' )
    ]


toRNA : String -> Result String String
toRNA =
    String.toList
        >> List.map (\c -> Dict.get c complement)
        >> List.foldr (Maybe.map2 String.cons) (Just "")
        >> Result.fromMaybe "Invalid input"
