Module Main
	{ Export = main
	; Import 
		= Foo
		& Bar [bar] as B
		& Bazz
	} where

{ Type Person 
	= name : Txt
	& age  : Nat

; Type Color
	= Blue
	| Red
	| Green
	| Custom Int8 
	       & Int8 
               & Int8

; Trait T : Equivalent where
	{ check_if_equal  : { T & T } -> Bool
	; chech_not_equal : { T & T } -> Bool
	}

; Impl Color : Equivalent where 
	{ check_if_qual = \case 
		{ { Blue & Blue }         = True   
		; { Red & Red }           = True   
		; { Green & Green }       = True   
		; { Custom n & Custom m } = n ?= m
		; _                       = False
      		}
	; check_not_equal = not check_if_equal
	}
; msg : Txt = "Hello world!"

; main : IO {} = do 
	{ println msg 
	; user_in <- get_line # get user input #
	; case user_in 
		{ "Hi!"             = println "Hello!"
		; "Bye" | "Goodbye" = println "Bye!"
		; _                 = return {}
		}
        ; let { msg_len = len user_in
	      ; foo = 3             
	      ; person = Person 
		  { name = "Tom" 
		  ; age  = 21
		  }
              }
        ; if { msg_len > 10 = println! "Stop yappin"
             ; msg_len < 2  = println! "Message is short"
	     ; True         = return {}                   
             }
        ; println! msg_len
	}
