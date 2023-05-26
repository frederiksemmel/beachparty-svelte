<script lang="ts">
	import { onMount } from 'svelte';

	import Section from '../lib/section.svelte';

	import beach from '$lib/assets/beach.png?w=1920&format=webp';
	import golf from '$lib/assets/golf.jpg?w=1920&format=webp';
	import tennis from '$lib/assets/tennis.jpg?w=1920&format=webp';
	import sailing from '$lib/assets/sailing.jpg?w=1920&format=webp';
	import sunbathing from '$lib/assets/sunbathing.jpg?w=1920&format=webp';
	import modernismo from '$lib/assets/modernismo.jpg?w=1920&format=webp';
	import caleta from '$lib/assets/caleta.jpg?w=1920&format=webp';

	var data = {"Tennis": [], "Golf": []};

	function format_date(date_str: string): string {
		var date = new Date(date_str);
		let hours = date.getHours();
		return `${hours}${hours >= 12 ? 'pm' : 'am'}`;
	}

	onMount(async () => {
		const recaptchaScript = document.createElement('script');
		recaptchaScript.setAttribute('src', 'https://assets.calendly.com/assets/external/widget.js');
		document.head.appendChild(recaptchaScript);

		const res = await fetch(`/api/events`);
		data = await res.json()
		console.log(data)
	});
</script>

<Section background={beach} dir="left">
	<div slot="title" class="font-bold">
		BEACHPARTY <br />
		15-17 SEPT 2023 <br />
		SANT ANDREU DE LLAVANERAS <br />
	</div>
	<div slot="content" class="flex flex-col align-start md:uppercase">
		<span class="font-bold my-4 text-amber-100">Sat Sept 16, 2023</span>
		<div class="grid grid-cols-2 grid-cols-[85px_auto] place-content-start gap-x-8">
			<span class="text-right text-amber-100">9 - 19PM</span>
			<span class=""
				>Golf / Tennis / Segeln / Sonnen / Spaziergang / <br /> Altersgerechte gymnastik</span
			>
			<span class="text-right text-amber-100">14PM</span>
			<span>Watering hole: La Caleta</span>
			<span class="text-right text-amber-100">15PM</span>
			<span>Lange Siesta</span>
			<span class="text-right text-amber-100">20PM</span>
			<span>Beachparty im GAS Quiet Club</span>
		</div>
		<span class="font-bold my-4 text-amber-100">Sun Sept 17, 2023</span>
		<div class="grid grid-cols-2 grid-cols-[85px_auto] place-content-start gap-x-8">
			<span class="text-right text-amber-100">10:30AM</span>
			<span class="">Café & Croissant, Petit Moll</span>
		</div>
	</div>
</Section>

<Section background={golf}>
	<div slot="title">
		GOLF <br />
		9:30 / 9:40 / 9:50
	</div>
	<div slot="content">
		Abschlag am Tee 10 – nur für Frühaufsteher <br />
		Dürfen wir Euch bitten, die Greenfee selber zu bezahlen? <br />
		Location: Club de Golf de Llavaneres, Camí Golf, 49-51, Sant Andreu de Llavaneres
	</div>
</Section>

<Section background={tennis} dir="left">
	<div slot="title">
		Tennis <br />
		12:00 / 13:00
	</div>
	<div slot="content">
		Einzel oder Doppel <br />
		Location: Tenis Mora, Camí de Can Pi, Sant Andreu de Llavaneres <br />
	</div>
</Section>

<Section background={sailing} dir="right">
	<div slot="title">
		Segeln <br />
		12:00 - 14:00
	</div>
	<div slot="content">
		Location: Club Náutico El Balís, Sant Andreu de Llavaneres <br />
		Liegeplatz: 468 - 470
	</div>
</Section>

<Section background={sunbathing} dir="left" gradient={false}>
	<div slot="title" class="text-slate-800">
		Sunbathing <br />
		12:00 - 18:00
	</div>
	<div slot="content" class="text-black">
		Vor Chiringuito Ohnades, bei schlechtem Wetter am Swimmingpool von Can Markus. <br />
		Locations: <br />
		Chiringuito Ohnades, Passeig del Marquès de Casa Riera, 25, 08394 Sant Vicenç de Montalt <br />
		“Can Markus”, Avinguda Turo d’en Llull 64, 08392 Sant Andreu de Llavaneres
	</div>
</Section>

<Section background={modernismo} dir="right">
	<div slot="title">
		Spaziergang <br />
		12:00
	</div>
	<div slot="content">Spaziergang mit katalanischem Modernismus in Canet de Mar</div>
</Section>

<Section background={caleta} dir="right">
	<div slot="title">
		Watering hole & Wine spot <br />
		Ab 14:00
	</div>
	<div slot="content">
		Location: La Caleta, Passeig del Marquès de Casa Riera, 45, Sant Vicenç de Montalt, <br />
		+34 937 911 558
	</div>
</Section>

<div class="my-24 mx-6 md:mx-40 flex flex-col space-y-6">
	<h2 class="text-4xl font-display">Aktivitäten</h2>
	{#each ['Tennis', 'Golf'] as event_type}
		<div class="mx-6 md:mx-40">
			<span class="text-xl font-bold">{event_type}</span>
			<div class="grid grid-cols-2 grid-cols-[60px_auto] gap-x-4">
				{#each data[event_type] as event}
					<span class="text-right"> {format_date(event['start'])}:</span>
					<div class="">
						{event['attendees'].join(', ')}
					</div>
				{/each}
			</div>
		</div>
	{/each}
	<!-- Calendly inline widget begin -->
	<div
		class="calendly-inline-widget min-w-[320px] h-[700px]"
		data-url="https://calendly.com/semmelbeachparty?hide_gdpr_banner=0"
	/>
	<!-- Calendly inline widget end -->
</div>
