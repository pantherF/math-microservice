async function calculate(operation) {
    const operand1 = document.getElementById('operand1').value;
    const operand2 = document.getElementById('operand2').value;

    if (!operand1 || !operand2) {
        document.getElementById('result').innerText = 'Please enter both operands.';
        return;
    }

    const requestBody = {
        operand1: parseFloat(operand1),
        operand2: parseFloat(operand2)
    };

    try {
        const response = await fetch(`http://localhost:8000/${operation}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestBody)
        });

        if (!response.ok) {
            const errorText = await response.text();
            document.getElementById('result').innerText = `Error: ${errorText}`;
            return;
        }

        const data = await response.json();
        document.getElementById('result').innerText = `Result: ${data.result}`;

    } catch (error) {
        document.getElementById('result').innerText = `Error: ${error.message}`;
    }
}