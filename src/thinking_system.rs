use super::prelude::*;

pub fn thinking_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut Status)>,
) {
    for (entity, mut brain, mut status) in query.iter_mut() {
        // if does_thinking.thinking {
        //     continue;
        // }
        // does_thinking.thinking = true;
        // CHOOSE A MOTIVATION
        // 0. Is having a crisis.
        // 1. Has been given an order AND there's a danger or emergency.
        // Needs order: Food, Health, Sleep, Entertainment.
        // 2. Has been given an order AND there's a matching need.
        // Is sick or injured and is trying to go to the hospital.
        // 3. Has an urgently unmet need.
        // 4. Has been given an order.
        // Based on personality:
        // Has meaningful work available to do.
        // Has a desire.
        // Wants to socialize or be entertained.
        
        // THEN ASSIGN A TASK BASED ON THE MOTIVATION
        if let Some(m) = &brain.task {
            continue; // Already has a task.
        }
        if let None = brain.motivation {
            // SET MOTIVATION
            if let Some(crisis) = &status.crisis {
                brain.motivation = Some(Motivation::Crisis);
            // Process dangers.
            } else if let Some(danger) = &status.danger {
                if let Some(order) = &brain.order {
                    brain.motivation = Some(Motivation::Order);
                } else {
                    brain.motivation = Some(Motivation::Danger);
                }
            // Process needs.
            }
            // FOOD
            if let None = brain.motivation {
                if let Some(n) = &status.needs_food {
                    if n.current < 5.0 {
                        if let Some(order) = &brain.order {
                            if order == "Eat" {
                                brain.motivation = Some(Motivation::Order);
                            } else {
                                brain.motivation = Some(Motivation::Hunger);
                            }
                        } else {
                            brain.motivation = Some(Motivation::Hunger);
                        }
                    }
                }
            }
            // HOSPITAL
            if let None = brain.motivation {
                if status.injured {
                    if let Some(order) = &brain.order {
                        if order == "Hospital" {
                            brain.motivation = Some(Motivation::Order);
                        } else {
                            brain.motivation = Some(Motivation::Injured);
                        }
                    } else {
                        brain.motivation = Some(Motivation::Injured);
                    }
                }
            }
            // SLEEP
            if let None = brain.motivation {
                if let Some(n) = &status.needs_sleep {
                    if n.current < 5.0 {
                        if let Some(order) = &brain.order {
                            if order == "Sleep" {
                                brain.motivation = Some(Motivation::Order);
                            } else {
                                brain.motivation = Some(Motivation::Tired);
                            }
                        } else {
                            brain.motivation = Some(Motivation::Tired);
                        }
                    }
                }
            }
            // ENTERTAINMENT
            if let None = brain.motivation {
                if let Some(n) = &status.needs_entertainment {
                    if n.current < 5.0 {
                        if let Some(order) = &brain.order {
                            if order == "Entertainment" {
                                brain.motivation = Some(Motivation::Order);
                            } else {
                                brain.motivation = Some(Motivation::Bored);
                            }
                        } else {
                            brain.motivation = Some(Motivation::Bored);
                        }
                    }
                }
            }
            // ORDERS
            if let None = brain.motivation {
                if let Some(order) = &brain.order {
                    brain.motivation = Some(Motivation::Order);
                }
            }
            // MEANINGFUL WORK
            if let None = brain.motivation {
                brain.motivation = Some(Motivation::Work);
            }
        }
        if let None = brain.motivation {
            brain.motivation = Some(Motivation::Meander);
        }
        // SET TASK
        if let Some(m) = brain.motivation {
            if m == Motivation::Crisis {
                if let Some(crisis) = &status.crisis {
                    // TO DO: Assign task based on crisis.
                    // if let Some(task) = &crisis.task {
                    //     brain.task = Some(task.to_string());
                    // }
                }
            } else if m == Motivation::Order {
                if let Some(order) = &brain.order {
                    brain.task = Some(Task::Order);
                }
            } else if m == Motivation::Danger {
                if let Some(danger) = &status.danger {
                    brain.task = Some(Task::Flee);
                    // TO DO: Assign FLEE or FIGHT task.
                }
            } else if m == Motivation::Hunger {
                brain.task = Some(Task::Eat);
            } else if m == Motivation::Injured {
                brain.task = Some(Task::Hospital);
            } else if m == Motivation::Tired {
                brain.task = Some(Task::Sleep);
            } else if m == Motivation::Bored {
                brain.task = Some(Task::Play);
            } else if m == Motivation::Work {
                brain.task = Some(Task::Work);
            } else if m == Motivation::Meander {
                brain.task = Some(Task::Meander);
            }
        }
        if let Some(task) = &brain.task {
            println!("{:?} has a task: {:?}", entity, brain.task)
        }
        
    }
}
pub fn remotivate_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain)>,
) {
    for (entity, mut brain) in query.iter_mut() {
        if let Some(m) = brain.task {
            if m == Task::Work || m == Task::Play || m == Task::Meander {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
    
}