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
