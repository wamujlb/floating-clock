export const initSettings: App.Settings = {
	showSeconds: true,
	opacity: 1,
	clockSize: 50,
	variant: 'Flip',
	pomodoro: {
		showPomodoro: true,
		focusTime: 25,
		interval: 30,
	},
};

export const roundToNearestEven = (num: number): number => Math.round((num * 0.4) / 2) * 2;

export const getInputValue = (currentTarget: EventTarget & HTMLInputElement) => {
	switch (currentTarget.type) {
		case 'checkbox':
			return currentTarget.checked;
		case 'number':
		case 'range':
			return currentTarget.valueAsNumber;
		default:
			return currentTarget.value;
	}
};
