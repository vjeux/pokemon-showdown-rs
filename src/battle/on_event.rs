use crate::*;
use crate::battle::EventContext;

impl Battle {
    /// Register a custom event handler (for testing)
    /// JavaScript: onEvent(eventid: string, target: Format, ...rest: AnyObject[])
    ///
    /// # Arguments
    /// * `event_id` - Event name (e.g., "Hit", "ModifyDamage")
    /// * `callback` - Function to call when event fires
    ///
    /// # Example
    /// ```ignore
    /// battle.on_event("Hit", |ctx| {
    ///     println!("Hit event on {:?}", ctx.target);
    ///     None // Return None for no value, Some(n) to return a value
    /// });
    /// ```
    // TypeScript source:
    // /**
    // 	 * Use this function to attach custom event handlers to a battle. See Battle#runEvent for
    // 	 * more information on how to write callbacks for event handlers.
    // 	 *
    // 	 * Try to use this sparingly. Most event handlers can be simply placed in a format instead.
    // 	 *
    // 	 *     this.onEvent(eventid, target, callback)
    // 	 * will set the callback as an event handler for the target when eventid is called with the
    // 	 * default priority. Currently only valid formats are supported as targets but this will
    // 	 * eventually be expanded to support other target types.
    // 	 *
    // 	 *     this.onEvent(eventid, target, priority, callback)
    // 	 * will set the callback as an event handler for the target when eventid is called with the
    // 	 * provided priority. Priority can either be a number or an object that contains the priority,
    // 	 * order, and subOrder for the event handler as needed (undefined keys will use default values)
    // 	 */
    // 	onEvent(eventid: string, target: Format, ...rest: AnyObject[]) { // rest = [priority, callback]
    // 		if (!eventid) throw new TypeError("Event handlers must have an event to listen to");
    // 		if (!target) throw new TypeError("Event handlers must have a target");
    // 		if (!rest.length) throw new TypeError("Event handlers must have a callback");
    //
    // 		if (target.effectType !== 'Format') {
    // 			throw new TypeError(`${target.name} is a ${target.effectType} but only Format targets are supported right now`);
    // 		}
    //
    // 		let callback, priority, order, subOrder, data;
    // 		if (rest.length === 1) {
    // 			[callback] = rest;
    // 			priority = 0;
    // 			order = false;
    // 			subOrder = 0;
    // 		} else {
    // 			[data, callback] = rest;
    // 			if (typeof data === 'object') {
    // 				priority = data['priority'] || 0;
    // 				order = data['order'] || false;
    // 				subOrder = data['subOrder'] || 0;
    // 			} else {
    // 				priority = data || 0;
    // 				order = false;
    // 				subOrder = 0;
    // 			}
    // 		}
    //
    // 		const eventHandler = { callback, target, priority, order, subOrder };
    //
    // 		if (!this.events) this.events = {};
    // 		const callbackName = `on${eventid}`;
    // 		const eventHandlers = this.events[callbackName];
    // 		if (eventHandlers === undefined) {
    // 			this.events[callbackName] = [eventHandler];
    // 		} else {
    // 			eventHandlers.push(eventHandler);
    // 		}
    // 	}
    //
    pub fn on_event<F>(&mut self, event_id: &str, callback: F)
    where
        F: Fn(&EventContext) -> Option<i32> + Send + Sync + 'static,
    {
        self.on_event_priority(event_id, 0, callback);
    }
}
