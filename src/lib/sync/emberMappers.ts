import type { Ember } from '$lib/db/types';
import type { ApiEmber } from '$lib/api/embers';

export function localToApiEmber(ember: Ember): ApiEmber {
	return {
		id: ember.id,
		name: ember.name,
		hours_today: ember.hoursToday,
		hours_this_week: ember.hoursThisWeek,
		hours_total: ember.hoursTotal,
		created_at: ember.createdAt,
		updated_at: ember.updatedAt,
		deleted_at: "" 
	};
}

export function apiToLocalEmber(ember: ApiEmber): Ember {
	return {
		id: ember.id,
		name: ember.name,
		hoursToday: ember.hours_today,
		hoursThisWeek: ember.hours_this_week,
		hoursTotal: ember.hours_total,
		createdAt: ember.created_at,
		updatedAt: ember.updated_at
	};
}
