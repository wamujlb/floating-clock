export type TimerPhase = 'idle' | 'focus' | 'break';

export type TimerState = {
	started: boolean;
	tick: number;
	phase: TimerPhase;
};
