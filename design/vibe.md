# Vibe Client

## Architecture

This could be considered as a single page application with router.

- `routes/ProjectView`: Project name and server info
- `routes/TracksView`: Track composing and controlling
- `routes/PatternsView`: Pattern programming (midi & message)
- `routes/ControlsView`: One-off events and numerical sliders

Pattern programming is split into two parts:

- `components/MidiProgramPane`: Midi programming (the piano row)
- `components/MessageProgramPane`: Message programming (re-fire-able events)

## Design Decisions

### Choosing Vue.JS

It's the only frontend framework I'm familiar with, with a decent ecosystem.

This turns out to be a "not very good" decision as it is hard to separate logics
from each other using single page component and with no state management library.
I don't know if this is a skill issue of mine or Vue's design flaw, maybe I should
try out more frontend frameworks and get better to answer this question.

That said, I don't regret choosing Vue as PrimeVue and VueUse makes prototyping
quick and easy.

### Use of External Libraries

- **Vue Router**: Basically to keep pages organized. This is a natural thought with
  the functionalities to be implemented.
- **Tailwind CSS**: A must, because I can't do CSS.
- **Lodash**: To make life easier, everyone knows what this is for.

### Not Using State Management Library

The original idea was to simplify the tech-stack and keep things easy and
minimal. I was going to go with Pinia but eventually declined that thought
because it adds a layer of "unneeded complexity", at least I thought.

This turns out to be a bad decision, and I am regretting. Handling such a
complex state with Vue's `provide()` and `inject()` is a nightmare, both
in code quality and in brain cell preservation.
