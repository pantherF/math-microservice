document.addEventListener('DOMContentLoaded', fetchHistory);

async function fetchHistory() {
    try {
        const response = await fetch('http://localhost:8000/calculations');
        
        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }
        
        const data = await response.json();
        displayHistory(data);
    } catch (error) {
        console.error('Error fetching history:', error);
        document.getElementById('historyBody').innerHTML = 
            `<tr><td colspan="5">Error loading history: ${error.message}</td></tr>`;
    }
}

function displayHistory(calculations) {
    const tableBody = document.getElementById('historyBody');
    tableBody.innerHTML = '';
    
    calculations.forEach(calc => {
        const row = document.createElement('tr');
        
        // Calculate the result based on the operation
        let result;
        switch(calc.operator) {
            case 'add':
                result = calc.first_number + calc.second_number;
                break;
            case 'subtract':
                result = calc.first_number - calc.second_number;
                break;
            case 'multiply':
                result = calc.first_number * calc.second_number;
                break;
            case 'divide':
                result = calc.first_number / calc.second_number;
                break;
            default:
                result = 'N/A';
        }
        
        row.innerHTML = `
            <td>${calc.id}</td>
            <td>${formatOperator(calc.operator)}</td>
            <td>${calc.first_number}</td>
            <td>${calc.second_number}</td>
            <td>${result}</td>
        `;
        
        tableBody.appendChild(row);
    });
}

function formatOperator(operator) {
    switch(operator) {
        case 'add': return '+';
        case 'subtract': return '-';
        case 'multiply': return '×';
        case 'divide': return '÷';
        default: return operator;
    }
}