// Function to show loading state
function showLoading() {
    document.getElementById('loading').classList.add('active');
    document.getElementById('error').classList.remove('active');
}

// Function to hide loading state
function hideLoading() {
    document.getElementById('loading').classList.remove('active');
}

// Function to show error message
function showError(message) {
    const errorDiv = document.getElementById('error');
    errorDiv.textContent = message;
    errorDiv.classList.add('active');
    hideLoading();
}

// Function to read the gas report data
function readGasReports(projectName) {
    try {
        const projectData = REPORTS_DATA[projectName];
        if (!projectData) {
            throw new Error(`No data available for project: ${projectName}`);
        }
        return {
            solcData: projectData.solc,
            solxData: projectData.solx
        };
    } catch (error) {
        showError(`Error reading gas reports: ${error.message}`);
        console.error('Error reading gas reports:', error);
        return null;
    }
}

// Color schemes for different compiler versions
const colorSchemes = {
    solc: {
        base: '#9C27B0', // Original purple
        getColor: (index, total) => {
            const baseHue = 291; // Purple hue
            const saturation = 76; // Fixed saturation
            // Adjust lightness from 45% to 65% based on index
            const lightness = 45 + (index * 10);
            return `hsl(${baseHue}, ${saturation}%, ${lightness}%)`;
        },
        getBorderColor: (color) => {
            // Extract HSL values from the color string
            const match = color.match(/hsl\((\d+),\s*(\d+)%,\s*(\d+)%\)/);
            if (!match) return color;
            const [, h, s, l] = match;
            // Return a darker version of the same color
            return `hsl(${h}, ${s}%, ${Math.max(20, parseInt(l) - 10)}%)`;
        }
    },
    solx: {
        base: '#2196F3', // Base blue
        getColor: (index, total) => {
            const baseHue = 207; // Blue hue
            const saturation = 90; // Fixed saturation
            // Adjust lightness from 45% to 65% based on index
            const lightness = 45 + (index * 10);
            return `hsl(${baseHue}, ${saturation}%, ${lightness}%)`;
        },
        getBorderColor: (color) => {
            // Extract HSL values from the color string
            const match = color.match(/hsl\((\d+),\s*(\d+)%,\s*(\d+)%\)/);
            if (!match) return color;
            const [, h, s, l] = match;
            // Return a darker version of the same color
            return `hsl(${h}, ${s}%, ${Math.max(20, parseInt(l) - 10)}%)`;
        }
    }
};

// Helper functions for color manipulation
function hexToRgb(hex) {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16)
    } : null;
}

function rgbToHsl(rgb) {
    const r = rgb.r / 255;
    const g = rgb.g / 255;
    const b = rgb.b / 255;
    
    const max = Math.max(r, g, b);
    const min = Math.min(r, g, b);
    let h, s, l = (max + min) / 2;

    if (max === min) {
        h = s = 0;
    } else {
        const d = max - min;
        s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
        switch (max) {
            case r: h = (g - b) / d + (g < b ? 6 : 0); break;
            case g: h = (b - r) / d + 2; break;
            case b: h = (r - g) / d + 4; break;
        }
        h /= 6;
    }

    return [h * 360, s * 100, l * 100];
}

// Function to calculate gas differences for a single contract
function calculateContractDiff(contractData) {
    const results = [];
    let methodCount = 0;

    // Find compiler settings objects and contract objects
    const settingsObjects = contractData.filter(obj => obj.compilerSettings && !obj.contract);
    const contractObjects = contractData.filter(obj => obj.contract);

    // Group data by function name across all compiler versions
    const functionData = {};
    
    contractObjects.forEach(version => {
        Object.entries(version.functions).forEach(([funcName, data]) => {
            if (!functionData[funcName]) {
                functionData[funcName] = [];
            }
            // Add compiler settings to the version data
            const settings = settingsObjects.find(s => s.compiler.type === version.compiler.type && 
                s.compiler.version === version.compiler.version)?.compilerSettings || {};
            functionData[funcName].push({
                compiler: version.compiler,
                gas: data.mean,
                compilerSettings: settings
            });
        });
    });

    // Find solx default version (without via-ir)
    const solxDefault = contractObjects.find(obj => 
        obj.compiler.type === 'solx' && 
        !obj.compiler.version.includes('via-ir')
    );

    if (!solxDefault) {
        console.error('No default solx version found');
        return null;
    }

    // Calculate differences for each function
    Object.entries(functionData).forEach(([funcName, versions]) => {
        const solxDefaultVersion = versions.find(v => 
            v.compiler.type === 'solx' && 
            !v.compiler.version.includes('via-ir')
        );

        if (solxDefaultVersion) {
            const baseGas = solxDefaultVersion.gas;
            results.push({
                name: funcName,
                versions: versions.filter(v => 
                    v === solxDefaultVersion || // Include solx default
                    v.compiler.type === 'solc' || // Include all solc versions
                    (v.compiler.type === 'solx' && v.compiler.version.includes('via-ir')) // Include solx via-ir
                ).map(v => ({
                    ...v,
                    baseGas,
                    difference: ((v.gas - baseGas) / baseGas * 100).toFixed(2)
                }))
            });
            methodCount++;
        }
    });

    // Get deployment info for all versions
    const deploymentInfo = contractObjects.map(version => {
        const settings = settingsObjects.find(s => 
            s.compiler.type === version.compiler.type && 
            s.compiler.version === version.compiler.version
        )?.compilerSettings || {};
        return {
            compiler: version.compiler,
            gas: version.deployment.gas,
            compilerSettings: settings
        };
    });

    // Sort deployment info to put solx default first
    const sortedDeployment = deploymentInfo.sort((a, b) => {
        if (a.compiler.type === 'solx' && !a.compiler.version.includes('via-ir')) return -1;
        if (b.compiler.type === 'solx' && !b.compiler.version.includes('via-ir')) return 1;
        return 0;
    });

    return {
        results,
        deployment: sortedDeployment,
        methodCount,
        contractName: solxDefault.contract
    };
}

// Function to calculate gas differences
function calculateGasDiff(solcData, solxData) {
    if (!solcData || !solxData) {
        showError('Invalid compiler data provided');
        return [];
    }

    // Group contracts by their name, filtering out objects without contract property
    const contractGroups = {};
    
    [...solcData, ...solxData].forEach(item => {
        if (!item.contract) return; // Skip items without contract property
        
        if (!contractGroups[item.contract]) {
            contractGroups[item.contract] = [];
        }
        // Add all items from the same compiler type to the group
        const compilerType = item.compiler.type;
        const settingsObject = [...solcData, ...solxData].find(
            s => s.compilerSettings && !s.contract && s.compiler.type === compilerType
        );
        
        contractGroups[item.contract].push(item);
        if (settingsObject) {
            contractGroups[item.contract].push(settingsObject);
        }
    });

    // Process each contract group
    return Object.values(contractGroups).map(contractData => 
        calculateContractDiff(contractData)
    );
}

let charts = [];

// Function to create a chart container
function createChartContainer(contractName, index) {
    const container = document.createElement('div');
    container.className = 'contract-section';
    container.innerHTML = `
        <h3>${contractName}</h3>
        <div class="chart-container">
            <canvas id="gasChart${index}"></canvas>
        </div>
        <div class="summary" id="summary${index}"></div>
    `;
    return container;
}

// Function to create a chart for a single contract
function createChart(data, index) {
    const ctx = document.getElementById(`gasChart${index}`).getContext('2d');
    
    // Sort results by maximum gas usage
    const sortedResults = [...data.results].sort((a, b) => {
        const maxA = Math.max(...a.versions.map(v => Number(v.gas)));
        const maxB = Math.max(...b.versions.map(v => Number(v.gas)));
        return maxB - maxA;
    });

    // Calculate dynamic height based on number of methods
    // Base height of 400px for up to 10 methods
    // Add 40px for each additional method
    const baseHeight = 400;
    const heightPerMethod = 40;
    const minMethods = 10;
    const methodCount = sortedResults.length;
    const dynamicHeight = methodCount <= minMethods ? 
        baseHeight : 
        baseHeight + (methodCount - minMethods) * heightPerMethod;

    // Set container height
    const container = ctx.canvas.parentElement;
    container.style.height = `${dynamicHeight}px`;

    // Create datasets for each compiler version
    const datasets = [];
    const allVersions = data.results[0].versions;
    
    allVersions.forEach((version, vIndex) => {
        const compilerType = version.compiler.type;
        const colorScheme = colorSchemes[compilerType];
        const backgroundColor = colorScheme.getColor(vIndex, allVersions.length);
        
        // Create dataset for this version
        const dataset = {
            label: `${compilerType} ${version.compiler.version}`,
            backgroundColor,
            borderColor: colorScheme.getBorderColor(backgroundColor),
            borderWidth: 1,
            barPercentage: 0.95,
            categoryPercentage: 0.8,
            data: []
        };

        // Fill in data for each method in sorted order
        sortedResults.forEach(method => {
            const matchingVersion = method.versions.find(v => 
                v.compiler.type === version.compiler.type && 
                v.compiler.version === version.compiler.version
            );
            dataset.data.push(matchingVersion ? Number(matchingVersion.gas) : 0);
        });

        datasets.push(dataset);
    });

    // Create chart
    return new Chart(ctx, {
        type: 'bar',
        data: {
            labels: sortedResults.map(r => r.name),
            datasets
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            indexAxis: 'y',
            plugins: {
                title: {
                    display: true,
                    text: 'Gas Usage Comparison by Method',
                    font: { size: 14 }
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            const methodName = sortedResults[context.dataIndex].name;
                            const version = allVersions[context.datasetIndex];
                            const matchingVersion = sortedResults[context.dataIndex].versions.find(v => 
                                v.compiler.type === version.compiler.type && 
                                v.compiler.version === version.compiler.version
                            );
                            const gas = matchingVersion ? Number(matchingVersion.gas).toLocaleString() : '0';
                            return `${version.compiler.type} ${version.compiler.version}: ${gas} gas`;
                        }
                    }
                }
            },
            scales: {
                y: {
                    beginAtZero: false,
                    grid: { display: false }
                },
                x: {
                    type: 'linear',
                    beginAtZero: true,
                    grid: { color: '#e0e0e0' },
                    title: {
                        display: true,
                        text: 'Gas Usage',
                        font: { size: 12 }
                    },
                    ticks: {
                        callback: function(value) {
                            return Number(value).toLocaleString();
                        }
                    }
                }
            }
        }
    });
}

// Function to update summary for a single contract
function updateSummary(data, index) {
    const summary = document.getElementById(`summary${index}`);
    
    // Sort deployment data to ensure solx default is first
    const sortedDeployment = [...data.deployment];
    const baseGas = sortedDeployment[0].gas;
    
    const deploymentHtml = sortedDeployment.map((version, idx) => {
        const percentageDiff = idx === 0 ? '' : 
            ` <span class="${baseGas > version.gas ? 'text-green-500' : 'text-red-500'} font-medium">(${((version.gas - baseGas) / baseGas * 100).toFixed(1)}%)</span>`;
            
        // Add special background for solx rows
        const rowBgClass = version.compiler.type === 'solx' ? 
            'bg-blue-50 border border-blue-100' : 
            'bg-gray-50';
            
        return `
            <div class="flex justify-between items-center ${rowBgClass} rounded-lg px-4 py-3 font-mono text-sm">
                <span class="${version.compiler.type === 'solx' ? 'text-solx font-medium' : 'text-gray-700'}">${version.compiler.type} ${version.compiler.version}</span>
                <div class="flex items-center gap-2">
                    <span class="text-gray-900 font-medium">${version.gas.toLocaleString()} gas</span>
                    ${percentageDiff}
                </div>
            </div>
        `;
    }).join('');

    // Group versions by compiler type
    const versions = {
        solx: data.results[0].versions.filter(v => v.compiler.type === 'solx'),
        solc: data.results[0].versions.filter(v => v.compiler.type === 'solc')
    };

    // Create version selector dropdowns
    const solxOptions = versions.solx.map(v => 
        `<option value="${v.compiler.version}" ${!v.compiler.version.includes('via-ir') ? 'selected' : ''}>${v.compiler.version}</option>`
    ).join('');

    const solcOptions = versions.solc.map((v, idx) => 
        `<option value="${v.compiler.version}" ${idx === 0 ? 'selected' : ''}>${v.compiler.version}</option>`
    ).join('');

    // Function to generate method comparisons
    function generateMethodComparisons(solxVersion, solcVersion) {
        const methodComparisons = [];
        data.results.forEach(method => {
            const solxData = method.versions.find(v => 
                v.compiler.type === 'solx' && 
                v.compiler.version === solxVersion
            );
            const solcData = method.versions.find(v => 
                v.compiler.type === 'solc' && 
                v.compiler.version === solcVersion
            );

            if (solxData && solcData) {
                const baseGas = solxData.gas;
                const savings = solcData.gas - baseGas;
                const savingsPercent = (savings / baseGas) * 100;
                if (Math.abs(savingsPercent) > 1) {  // Only show differences > 1%
                    methodComparisons.push({
                        name: method.name,
                        savings,
                        savingsPercent,
                        baseGas: baseGas,
                        comparedGas: solcData.gas
                    });
                }
            }
        });
        return methodComparisons;
    }

    // Create the summary HTML
    summary.innerHTML = `
        <div class="space-y-6">
            <div class="bg-white rounded-xl shadow-sm overflow-hidden">
                <div class="border-b border-gray-200">
                    <h4 class="text-lg font-semibold text-gray-900 px-6 py-4">Contract Deployment</h4>
                </div>
                <div class="p-6 space-y-3">
                    ${deploymentHtml}
                </div>
            </div>

            <div class="bg-white rounded-xl shadow-sm overflow-hidden">
                <div class="border-b border-gray-200">
                    <h4 class="text-lg font-semibold text-gray-900 px-6 py-4">Methods Summary</h4>
                </div>
                <div class="p-6">
                    <div class="text-gray-600 pb-4 mb-6 border-b border-gray-200">
                        Total Methods Tested: ${data.methodCount}
                    </div>
                    <div class="flex gap-4 mb-6">
                        <div class="flex-1">
                            <label class="block text-sm font-medium text-gray-700 mb-2">solx version</label>
                            <select class="version-select w-full px-3 py-2 rounded-lg border border-gray-300" data-compiler="solx">
                                ${solxOptions}
                            </select>
                        </div>
                        <div class="flex-1">
                            <label class="block text-sm font-medium text-gray-700 mb-2">solc version</label>
                            <select class="version-select w-full px-3 py-2 rounded-lg border border-gray-300" data-compiler="solc">
                                ${solcOptions}
                            </select>
                        </div>
                    </div>
                    <div id="methodComparisons${index}" class="space-y-2">
                    </div>
                </div>
            </div>
        </div>
    `;

    // Function to update method comparisons
    function updateMethodComparisons() {
        const solxVersion = summary.querySelector('[data-compiler="solx"]').value;
        const solcVersion = summary.querySelector('[data-compiler="solc"]').value;
        const comparisons = generateMethodComparisons(solxVersion, solcVersion);
        
        // Sort by absolute percentage difference
        comparisons.sort((a, b) => Math.abs(b.savingsPercent) - Math.abs(a.savingsPercent));

        const methodComparisonsHtml = comparisons.map(m => `
            <div class="flex justify-between items-center bg-white rounded-lg p-3 font-mono text-sm border border-gray-100">
                <span class="text-gray-900">${m.name}</span>
                <div class="flex items-center gap-3">
                    <span class="text-solx">${m.baseGas.toLocaleString()}</span>
                    <span class="text-gray-400">→</span>
                    <span class="text-solc">${m.comparedGas.toLocaleString()}</span>
                    <span class="${m.savingsPercent > 0 ? 'text-red-500' : 'text-green-500'}">(${m.savingsPercent > 0 ? '+' : ''}${m.savingsPercent.toFixed(2)}%)</span>
                </div>
            </div>
        `).join('');

        const methodComparisonsContainer = document.getElementById(`methodComparisons${index}`);
        methodComparisonsContainer.innerHTML = methodComparisonsHtml || '<div class="text-gray-500 text-center py-4">No significant differences found</div>';
    }

    // Add event listeners to dropdowns
    summary.querySelectorAll('.version-select').forEach(select => {
        select.addEventListener('change', updateMethodComparisons);
    });

    // Initial update
    updateMethodComparisons();
}

// Function to update the dashboard
function updateChart(data) {
    if (!data || !Array.isArray(data) || data.length === 0) {
        showError('No valid data to display');
        return;
    }

    // Clear existing charts
    charts.forEach(chart => chart.destroy());
    charts = [];
    
    // Get the chart container
    const container = document.getElementById('chartContainer');
    if (!container) {
        console.error('Chart container not found');
        return;
    }
    
    // Clear the container
    container.innerHTML = '';

    // Create sections for each contract
    data.forEach((contractData, index) => {
        // Create and append container for this contract
        const contractSection = createChartContainer(contractData.contractName, index);
        container.appendChild(contractSection);

        // Create chart and update summary
        const chart = createChart(contractData, index);
        if (chart) {
            charts.push(chart);
            updateSummary(contractData, index);
        }
    });
}

// Function to populate project dropdown
function populateProjects() {
    // Get list of available projects from the reports data
    const projects = Object.keys(REPORTS_DATA);
    
    const select = document.getElementById('projectSelect');
    select.innerHTML = '<option value="">Select a project...</option>';
    
    projects.forEach(project => {
        const option = document.createElement('option');
        option.value = project;
        option.textContent = project;
        select.appendChild(option);
    });
}

// Event listener for project selection
document.getElementById('projectSelect').addEventListener('change', (event) => {
    const projectName = event.target.value;
    if (projectName) {
        showLoading();
        const reports = readGasReports(projectName);
        if (reports) {
            const data = calculateGasDiff(reports.solcData, reports.solxData);
            updateChart(data);
        }
        hideLoading();
    }
});

// Initialize the dashboard
document.addEventListener('DOMContentLoaded', () => {
    populateProjects();
}); 
