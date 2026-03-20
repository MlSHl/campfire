<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import Step from '$lib/components/footsteps/Step.svelte';
    import StepCard from '$lib/components/footsteps/StepCard.svelte';
    import AddAndClear from '$lib/components/footsteps/AddAndClear.svelte';

	import type { Footstep } from '$lib/db/types';
	import {
        toggleCompleteFootstep,
		getAllFootsteps,
		createFootstep,
		updateFootstep,
        deleteFootstep,
        getCompletedCount,
        deleteCompletedSteps
	} from '$lib/db/footsteps';

    let steps = $state<Footstep[]>([]);
    let isMobile = $state(false);
    let selectedStep = $state<Footstep | null>(null);
    let completedCount = $state(0);

    $effect(() => {
        if (typeof window === "undefined") return;

        const media = window.matchMedia('(max-width: 767px)');

        const update = () => isMobile = media.matches;

        update();
        media.addEventListener('change', update);

        return () => media.removeEventListener('change', update);
    });

    onMount(async () => {
		steps = await getAllFootsteps();
        completedCount = await getCompletedCount();
	});

    async function addFootstep() {
        let emptyStep: Footstep = await createFootstep({name: '', content: '', completed: 0});
        steps.push(emptyStep);
    }

    async function toggleComplete(id: string) {
        await toggleCompleteFootstep(id);
        let step = steps.find((step) => step.id === id);
        if (step) {
            if (step.completed) {
                completedCount--;
                step.completed = 0;
            } else {
                completedCount++;
                step.completed = 1;
            }
        }
    }

    function openDetails(step: Footstep) {
        if (isMobile) {
            goto(`/footsteps/${step.id}`);
            return;
        }

        selectedStep = step;
    }

    function closeDetails() {
        selectedStep = null;
    }

    async function saveSelectedStep() {
        if (selectedStep) {
            await updateFootstep(selectedStep);
        }
    }

    async function deleteSelectedStep() {
        if (selectedStep) {
            await deleteFootstep(selectedStep.id);
            selectedStep = null;
            steps = await getAllFootsteps();
        }
    }

    async function clearCompleted() {
        await deleteCompletedSteps();
        steps = await getAllFootsteps();
        completedCount = await getCompletedCount();
    }

</script>

<div class="relative md:h-[calc(100vh-5rem)]">
    {#each steps as step, i (step.id)}
        <Step bind:step={steps[i]} {toggleComplete} {updateFootstep} {openDetails}/>
    {/each}
    <AddAndClear {addFootstep}{clearCompleted }/>

    {#if selectedStep}
        <StepCard {selectedStep} {closeDetails} {saveSelectedStep} {deleteSelectedStep}/>
    {/if}
</div>
