<script lang="ts">
	/** @type {import('./$types').PageData} */
	export let data;

	// $: console.log(data)

	function format_date(date_str: string): string {
		var date = new Date(date_str);
	  let hours = date.getHours();
	  return `${hours}${hours >= 12 ? 'pm' : 'am'}`
	}

</script>

<h1 class="text-6xl">Semmels Beachparty</h1>
<h2 class="text-4xl">Programm</h2>
<div>
	{#each ["Tennis", "Golf"] as event_type}
	<div>
		<span class="text-xl">{event_type}</span>
		<div class="flex flex-col">
			{#each data[event_type] as event}
				<div class="flex flex-row">
				<span class="w-20"> Um {format_date(event["start"])}:</span>
				<div class="">
				{event["attendees"].join(", ")}
				</div>
				</div>
			{/each}
		</div>
	</div>

	{/each}
</div>

<!-- Calendly inline widget begin -->
<div
	class="calendly-inline-widget min-w-[320px] h-[700px]"
	data-url="https://calendly.com/semmelbeachparty?hide_gdpr_banner=0"
/>
<!-- Calendly inline widget end -->
