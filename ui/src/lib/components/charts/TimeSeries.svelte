<script>
  import { onMount } from 'svelte';
  import Chart from 'chart.js/auto';

  let portfolio;
  const data = {
        labels: ['Expenses', 'Savings', 'Investments'],
        datasets: [
            {
                label: 'My First Dataset',
                data: [300, 50, 100],
                backgroundColor: ['#7000e1', '#fc8800', '#00b0e8'],
                // hoverOffset: 4,
                borderWidth: 0
            }
        ]
    };
    const config = {
        type: 'doughnut',
        data: data,
        options: {
            borderRadius: '30',
            responsive: true,
            cutout: '95%',
            spacing: 2,
            plugins: {
                legend: {
                    position: 'bottom',
                    display: true,
                    labels: {
                        usePointStyle: true,
                        padding: 20,
                        font: {
                            size: 14
                        }
                    }
                },
                title: {
                    display: true,
                    text: 'My Personal Portfolio'
                }
            }
        }
    };
  onMount(()=> {
    const ctx = portfolio.getContext('2d');
    // Initialize chart using default config set
    var myChart = new Chart(ctx, config);
  })
</script>

<div>
    <canvas bind:this={portfolio} width={100} height={100} />
</div>


<style>
    div { width: 50vw; height: 50vh; }
    canvas {
        width: 100%;
        height: 100%;
        background-color: #666;
    }
</style>