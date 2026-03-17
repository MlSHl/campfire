<script lang="ts">
    import Step from '$lib/footsteps/Step.svelte';
    import type { StepType } from '$lib/types/step.type';
    let steps = $state<StepType[]>([
        {id: 1, name: "make UI", completed: false},
        {id: 2, name: "make features", completed: false},
        {id: 3, name: "make syncing", completed: false}
    ]);

    function addStep(step: StepType):  number {
        step.id = steps[-1].id + 1;
        steps.push(step);
        return step.id;
    }

    function updateStep(step: StepType, id:number) {
        steps[id] = step;
    }

    function completeStep(id: number) {
        const step = steps.find((step) => step.id === id);
        if (step) {
            step.completed = true;
        }
    }
</script>

{#each steps as step}
    <Step {step} {completeStep} {addStep} {updateStep}/>
{/each}

