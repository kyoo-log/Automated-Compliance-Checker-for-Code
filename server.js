
const express = require('express');
const { exec } = require('child_process');
const app = express();

app.use(express.json());

app.post('/check-compliance', (req, res) => {
    const { directory } = req.body;
    if (!directory) {
        return res.status(400).json({ error: 'Directory path is required.' });
    }

    exec(`cargo run -- ${directory}`, (error, stdout, stderr) => {
        if (error) {
            return res.status(500).json({ error: stderr.trim() });
        }
        res.json({ result: stdout.trim() });
    });
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`Server running on port ${PORT}`));
