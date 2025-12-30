use crate::*;

impl Battle {

    /// Add a log entry - matches JS this.add()
    /// JavaScript: add(...parts: (Part | (() => { side: SideID, secret: string, shared: string }))[])
    /// Usage: battle.add("-activate", &[pokemon.into(), "ability: Immunity".into()])
    //
    // 	add(...parts: (Part | (() => { side: SideID, secret: string, shared: string }))[]) {
    // 		if (!parts.some(part => typeof part === 'function')) {
    // 			this.log.push(`|${parts.join('|')}`);
    // 			return;
    // 		}
    //
    // 		let side: SideID | null = null;
    // 		const secret = [];
    // 		const shared = [];
    // 		for (const part of parts) {
    // 			if (typeof part === 'function') {
    // 				const split = part();
    // 				if (side && side !== split.side) throw new Error("Multiple sides passed to add");
    // 				side = split.side;
    // 				secret.push(split.secret);
    // 				shared.push(split.shared);
    // 			} else {
    // 				secret.push(part);
    // 				shared.push(part);
    // 			}
    // 		}
    // 		this.addSplit(side!, secret, shared);
    // 	}
    //
    pub fn add(&mut self, event_type: &str, args: &[Arg]) {
        // JS: if (!parts.some(part => typeof part === 'function'))
        let has_split_fn = args.iter().any(|arg| matches!(arg, Arg::SplitFn(_)));

        if !has_split_fn {
            // JS: this.log.push(`|${parts.join('|')}`);
            let mut entry = format!("|{}", event_type);
            for arg in args {
                entry.push('|');
                entry.push_str(&arg.to_string());
            }
            self.log.push(entry);
            return;
        }

        // JS: let side: SideID | null = null;
        let mut side: Option<SideID> = None;
        let mut secret = Vec::new();
        let mut shared = Vec::new();

        // Add event_type to both secret and shared
        secret.push(event_type.to_string());
        shared.push(event_type.to_string());

        // JS: for (const part of parts)
        for arg in args {
            match arg {
                Arg::SplitFn(func) => {
                    // JS: if (typeof part === 'function')
                    let split = func();

                    // JS: if (side && side !== split.side) throw new Error("Multiple sides passed to add");
                    if let Some(existing_side) = side {
                        if existing_side != split.side {
                            panic!("Multiple sides passed to add");
                        }
                    }

                    // JS: side = split.side;
                    side = Some(split.side);

                    // JS: secret.push(split.secret); shared.push(split.shared);
                    secret.push(split.secret);
                    shared.push(split.shared);
                }
                _ => {
                    // JS: secret.push(part); shared.push(part);
                    let s = arg.to_string();
                    secret.push(s.clone());
                    shared.push(s);
                }
            }
        }

        // JS: this.addSplit(side!, secret, shared);
        if let Some(side_id) = side {
            let side_str = match side_id {
                SideID::P1 => "p1",
                SideID::P2 => "p2",
                SideID::P3 => "p3",
                SideID::P4 => "p4",
            };
            self.add_split(
                side_str,
                &secret.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                Some(&shared.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
            );
        }
    }
}
