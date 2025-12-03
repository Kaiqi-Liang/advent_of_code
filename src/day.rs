#[macro_export]
macro_rules! define_days {
	($($name:ident => $num:literal)*) => {
		#[derive(Clone, ValueEnum)]
		pub enum Day {
			$(
    			#[value(name = stringify!($num))]
    			$name,
			)*
		}

		impl Display for Day {
		    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		        match self {
					$(
		            	Day::$name => f.write_str(stringify!($num)),
					)*
		        }
		    }
		}

		impl Day {
			pub fn run(&self, input: &str, part: Part) -> Result<String, Box<dyn Error>> {
				match self {
					$(
						Day::$name => {
							Ok(paste! { [<day $num>]::answer(input, part)?.to_string() })
						}
					)*
				}
			}
		}
	};
}
