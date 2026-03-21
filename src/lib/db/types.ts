export type Log = {
	id: string;
	name: string;
	content: string;
	createdAt: string;
	updatedAt: string;
    lastVisitedAt: string;
};

export type Footstep = {
	id: string;
	name: string;
	content: string;
    completed: 0 | 1;
	createdAt: string;
	updatedAt: string;
};

export type Ember = {
	id: string;
	name: string;
	hoursToday: number;
	hoursThisWeek: number;
	hoursTotal: number;
	createdAt: string;
	updatedAt: string;
};
