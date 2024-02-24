export const initSettings: App.Settings = {
	showSeconds: true,
	opacity: 1,
	clockSize: 50,
	variant: 'Flip',
};

export const roundToNearestEven = (num: number): number => Math.round((num * 0.4) / 2) * 2;
