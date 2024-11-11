use serde_json::Value;
use std::fs;
use std::collections::HashMap;
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting JSON to HTML conversion process...");

    let json_content = fs::read_to_string("query_results/alerts_Mini-060.local_1731360097.json")?;
    println!("JSON file read successfully.");

    let json: Value = serde_json::from_str(&json_content)?;
    println!("JSON parsed successfully.");

    let html_content = generate_html(&json)?;
    println!("HTML content generated.");
    
    let html_file_name = format!("wazuh_security_report_{}.html", Local::now().format("%Y%m%d_%H%M%S"));
    fs::write(&html_file_name, &html_content)?;
    println!("HTML security report generated: {}", html_file_name);

    Ok(())
}

fn generate_html(json: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("    <title>Wazuh Security Report</title>\n");
    html.push_str("    <script src=\"https://cdn.tailwindcss.com\"></script>\n");
    html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n");
    html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/apexcharts\"></script>\n");
    html.push_str("    <style>\n");
    html.push_str("        @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }\n");
    html.push_str("        .fade-in { animation: fadeIn 0.5s ease-in; }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n<body class=\"bg-gray-100\">\n");
    html.push_str("    <div class=\"container mx-auto px-4 py-8\">\n");
    html.push_str("        <h1 class=\"text-4xl font-bold text-center text-blue-600 mb-8\">Wazuh Security Report</h1>\n");
    
    if let Some(hits) = json["hits"]["hits"].as_array() {
        let mut alerts_by_level: HashMap<i64, Vec<&Value>> = HashMap::new();
        let mut alerts_by_rule: HashMap<String, usize> = HashMap::new();
        let mut agents: HashMap<String, usize> = HashMap::new();
        let mut mitre_techniques: HashMap<String, usize> = HashMap::new();
        let mut compliance_standards: HashMap<String, usize> = HashMap::new();
        let total_alerts = hits.len();

        for hit in hits {
            if let Some(source) = hit["_source"].as_object() {
                if let Some(rule) = source.get("rule") {
                    if let Some(level) = rule["level"].as_i64() {
                        alerts_by_level.entry(level).or_default().push(hit);
                    }
                    if let Some(description) = rule["description"].as_str() {
                        *alerts_by_rule.entry(description.to_string()).or_default() += 1;
                    }
                    if let Some(mitre) = rule.get("mitre") {
                        if let Some(techniques) = mitre["technique"].as_array() {
                            for technique in techniques {
                                if let Some(technique_str) = technique.as_str() {
                                    *mitre_techniques.entry(technique_str.to_string()).or_default() += 1;
                                }
                            }
                        }
                    }
                    // Count compliance standards
                    for standard in &["pci_dss", "hipaa", "gdpr", "nist_800_53", "tsc"] {
                        if let Some(values) = rule[standard].as_array() {
                            for value in values {
                                if let Some(value_str) = value.as_str() {
                                    *compliance_standards.entry(format!("{}: {}", standard, value_str)).or_default() += 1;
                                }
                            }
                        }
                    }
                }
                if let Some(agent) = source.get("agent") {
                    if let Some(name) = agent["name"].as_str() {
                        *agents.entry(name.to_string()).or_default() += 1;
                    }
                }
            }
        }

        let top_agent = agents.iter().max_by_key(|&(_, count)| count).map(|(name, count)| (name.clone(), *count));
        let (top_agent_name, top_agent_count) = top_agent.clone().map_or(("N/A".to_string(), 0), |(name, count)| (name, count));

        // Summary Section
        html.push_str("        <div class=\"bg-white rounded-lg shadow-md p-6 mb-8 fade-in\">\n");
        html.push_str("            <h2 class=\"text-2xl font-semibold mb-4\">Summary</h2>\n");
        html.push_str(&format!("            <p class=\"text-3xl font-bold text-blue-500\">Total Alerts: {}</p>\n", total_alerts));
        html.push_str(&format!("            <p class=\"text-sm text-gray-500 mt-2\">Generated on: {}</p>\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        html.push_str(&format!("            <p class=\"text-lg font-medium mt-4\">Top Agent: {} ({} alerts)</p>\n", top_agent_name, top_agent_count));
        html.push_str("        </div>\n");

        // Charts Section
        html.push_str("        <div class=\"grid grid-cols-1 md:grid-cols-2 gap-6 mb-8\">\n");
        
        // Alert Levels Distribution Chart
        html.push_str("            <div class=\"bg-white rounded-lg shadow-md p-6 fade-in\">\n");
        html.push_str("                <h2 class=\"text-2xl font-semibold mb-4\">Alert Levels Distribution</h2>\n");
        html.push_str("                <div id=\"alertLevelsChart\"></div>\n");
        html.push_str("            </div>\n");

        // Top Alert Types Chart
        html.push_str("            <div class=\"bg-white rounded-lg shadow-md p-6 fade-in\">\n");
        html.push_str("                <h2 class=\"text-2xl font-semibold mb-4\">Top Alert Types</h2>\n");
        html.push_str("                <canvas id=\"alertTypesChart\"></canvas>\n");
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");

        // MITRE ATT&CK Techniques
        html.push_str("        <div class=\"bg-white rounded-lg shadow-md p-6 mb-8 fade-in\">\n");
        html.push_str("            <h2 class=\"text-2xl font-semibold mb-4\">MITRE ATT&CK Techniques</h2>\n");
        html.push_str("            <div id=\"mitreChart\"></div>\n");
        html.push_str("        </div>\n");

        // Compliance Standards
        html.push_str("        <div class=\"bg-white rounded-lg shadow-md p-6 mb-8 fade-in\">\n");
        html.push_str("            <h2 class=\"text-2xl font-semibold mb-4\">Compliance Standards</h2>\n");
        html.push_str("            <div id=\"complianceChart\"></div>\n");
        html.push_str("        </div>\n");

        // Alerts Timeline
        html.push_str("        <div class=\"bg-white rounded-lg shadow-md p-6 mb-8 fade-in\">\n");
        html.push_str("            <h2 class=\"text-2xl font-semibold mb-4\">Alerts Timeline</h2>\n");
        html.push_str("            <div id=\"alertsTimeline\"></div>\n");
        html.push_str("        </div>\n");

        // Detailed Alerts Section
        let mut sorted_alerts: Vec<_> = alerts_by_level.into_iter().collect();
        sorted_alerts.sort_by(|a, b| b.0.cmp(&a.0));

        for (level, alerts) in &sorted_alerts {
            html.push_str(&format!("        <div class=\"bg-white rounded-lg shadow-md p-6 mb-8 fade-in\">\n"));
            html.push_str(&format!("            <h2 class=\"text-2xl font-semibold mb-4\">Level {} Alerts</h2>\n", level));
            html.push_str("            <div class=\"space-y-4\">\n");

            for (index, alert) in alerts.iter().enumerate() {
                if let Some(source) = alert["_source"].as_object() {
                    html.push_str(&format!("                <div class=\"border-l-4 border-blue-500 pl-4\">\n"));
                    html.push_str(&format!("                    <h3 class=\"text-lg font-semibold mb-2\">Alert {}</h3>\n", index + 1));
                    
                    if let Some(timestamp) = source.get("timestamp") {
                        html.push_str(&format!("                    <p class=\"text-sm text-gray-500 mb-2\">{}</p>\n", timestamp));
                    }
                    if let Some(agent) = source.get("agent") {
                        if let Some(agent_name) = agent["name"].as_str() {
                            html.push_str(&format!("                    <p class=\"mb-1\"><span class=\"font-medium\">Agent:</span> {}</p>\n", agent_name));
                        }
                    }
                    if let Some(rule) = source.get("rule") {
                        if let Some(description) = rule["description"].as_str() {
                            html.push_str(&format!("                    <p class=\"mb-1\"><span class=\"font-medium\">Description:</span> {}</p>\n", description));
                        }
                        if let Some(mitre) = rule.get("mitre") {
                            html.push_str("                    <p class=\"mb-1\"><span class=\"font-medium\">MITRE ATT&CK:</span></p>\n");
                            html.push_str("                    <ul class=\"list-disc list-inside pl-4\">\n");
                            if let Some(techniques) = mitre["technique"].as_array() {
                                for technique in techniques {
                                    html.push_str(&format!("                        <li>{}</li>\n", technique));
                                }
                            }
                            html.push_str("                    </ul>\n");
                        }
                    }
                    if let Some(data) = source.get("data") {
                        html.push_str("                    <div class=\"mt-2 text-sm text-gray-600\">\n");
                        html.push_str("                        <p class=\"font-medium\">Additional Data:</p>\n");
                        html.push_str("                        <ul class=\"list-disc list-inside pl-4\">\n");
                        for (key, value) in data.as_object().unwrap() {
                            html.push_str(&format!("                            <li><span class=\"font-medium\">{}</span>: {}</li>\n", key, value));
                        }
                        html.push_str("                        </ul>\n");
                        html.push_str("                    </div>\n");
                    }
                    html.push_str("                </div>\n");
                }
            }

            html.push_str("            </div>\n");
            html.push_str("        </div>\n");
        }

        // JavaScript for charts
        html.push_str("    <script>\n");

        // Alert Levels Distribution Chart
        html.push_str("        var alertLevelsOptions = {\n");
        html.push_str(&format!("            series: [{}],\n", sorted_alerts.iter().map(|(_, alerts)| alerts.len().to_string()).collect::<Vec<String>>().join(", ")));
        html.push_str("            chart: { type: 'bar', height: 350 },\n");
        html.push_str("            plotOptions: { bar: { borderRadius: 4, horizontal: true, } },\n");
        html.push_str("            dataLabels: { enabled: false },\n");
        html.push_str(&format!("            xaxis: {{ categories: [{}], }},\n", sorted_alerts.iter().map(|(level, _)| level.to_string()).collect::<Vec<String>>().join(", ")));
        html.push_str("            title: { text: 'Alert Levels Distribution', align: 'center' },\n");
        html.push_str(&format!("            colors: [{}]\n", sorted_alerts.iter().map(|(level, _)| format!("'rgba({}, {}, {}, 0.8)'", level * 20 % 255, level * 40 % 255, level * 60 % 255)).collect::<Vec<String>>().join(", ")));
        html.push_str("        };\n");
        html.push_str("        var alertLevelsChart = new ApexCharts(document.querySelector(\"#alertLevelsChart\"), alertLevelsOptions);\n");
        html.push_str("        alertLevelsChart.render();\n\n");

        // Top Alert Types Chart
        html.push_str("        var alertTypesCtx = document.getElementById('alertTypesChart').getContext('2d');\n");
        html.push_str("        new Chart(alertTypesCtx, {\n");
        html.push_str("            type: 'doughnut',\n");
        html.push_str("            data: {\n");
        html.push_str(&format!("                labels: [{}],\n", alerts_by_rule.keys().take(5).map(|k| format!("'{}'", k.replace("'", "\\'"))).collect::<Vec<String>>().join(", ")));
        html.push_str("                datasets: [{\n");
        html.push_str(&format!("                    data: [{}],\n", alerts_by_rule.values().take(5).map(|v| v.to_string()).collect::<Vec<String>>().join(", ")));
        html.push_str("                    backgroundColor: ['rgba(255, 99, 132, 0.8)', 'rgba(54, 162, 235, 0.8)', 'rgba(255, 206, 86, 0.8)', 'rgba(75, 192, 192, 0.8)', 'rgba(153, 102, 255, 0.8)'],\n");
        html.push_str("                    borderColor: ['rgba(255, 99, 132, 1)', 'rgba(54, 162, 235, 1)', 'rgba(255, 206, 86, 1)', 'rgba(75, 192, 192, 1)', 'rgba(153, 102, 255, 1)'],\n");
        html.push_str("                    borderWidth: 1\n");
        html.push_str("                }]\n");
        html.push_str("            },\n");
        html.push_str("            options: {\n");
        html.push_str("                responsive: true,\n");
        html.push_str("                plugins: {\n");
        html.push_str("                    legend: { position: 'right' },\n");
        html.push_str("                    title: { display: true, text: 'Top Alert Types' }\n");
        html.push_str("                }\n");
        html.push_str("            }\n");
        html.push_str("        });\n");

        // MITRE ATT&CK Techniques Chart
        html.push_str("        var mitreOptions = {\n");
        html.push_str("            series: [{\n");
        html.push_str(&format!("                data: [{}]\n", mitre_techniques.values().take(10).map(|v| v.to_string()).collect::<Vec<String>>().join(", ")));
        html.push_str("            }],\n");
        html.push_str("            chart: { type: 'bar', height: 350 },\n");
        html.push_str("            plotOptions: { bar: { borderRadius: 4, horizontal: true, } },\n");
        html.push_str("            dataLabels: { enabled: false },\n");
        html.push_str(&format!("            xaxis: {{ categories: [{}], }},\n", mitre_techniques.keys().take(10).map(|k| format!("'{}'", k.replace("'", "\\'"))).collect::<Vec<String>>().join(", ")));
        html.push_str("            title: { text: 'Top 10 MITRE ATT&CK Techniques', align: 'center' },\n");
        html.push_str("        };\n");
        html.push_str("        var mitreChart = new ApexCharts(document.querySelector(\"#mitreChart\"), mitreOptions);\n");
        html.push_str("        mitreChart.render();\n");

        // Compliance Standards Chart
        html.push_str("        var complianceOptions = {\n");
        html.push_str("            series: [{\n");
        html.push_str(&format!("                data: [{}]\n", compliance_standards.values().take(10).map(|v| v.to_string()).collect::<Vec<String>>().join(", ")));
        html.push_str("            }],\n");
        html.push_str("            chart: { type: 'bar', height: 350 },\n");
        html.push_str("            plotOptions: { bar: { borderRadius: 4, horizontal: true, } },\n");
        html.push_str("            dataLabels: { enabled: false },\n");
        html.push_str(&format!("            xaxis: {{ categories: [{}], }},\n", compliance_standards.keys().take(10).map(|k| format!("'{}'", k.replace("'", "\\'"))).collect::<Vec<String>>().join(", ")));
        html.push_str("            title: { text: 'Top 10 Compliance Standards', align: 'center' },\n");
        html.push_str("        };\n");
        html.push_str("        var complianceChart = new ApexCharts(document.querySelector(\"#complianceChart\"), complianceOptions);\n");
        html.push_str("        complianceChart.render();\n");

        // Alerts Timeline
        html.push_str("        var timelineOptions = {\n");
        html.push_str("            series: [{\n");
        html.push_str("                data: [\n");
        for hit in hits.iter().take(50) {
            if let Some(source) = hit["_source"].as_object() {
                if let (Some(timestamp), Some(rule)) = (source.get("timestamp"), source.get("rule")) {
                    if let (Some(timestamp), Some(level), Some(description)) = (timestamp.as_str(), rule["level"].as_i64(), rule["description"].as_str()) {
                        html.push_str(&format!("                    {{ x: new Date('{}').getTime(), y: {}, description: '{}' }},\n", timestamp, level, description.replace("'", "\\'")));
                    }
                }
            }
        }
        html.push_str("                ]\n");
        html.push_str("            }],\n");
        html.push_str("            chart: { type: 'scatter', height: 350, zoom: { type: 'xy' } },\n");
        html.push_str("            xaxis: { type: 'datetime' },\n");
        html.push_str("            yaxis: { title: { text: 'Alert Level' } },\n");
        html.push_str("            title: { text: 'Alerts Timeline (Last 50 Alerts)', align: 'center' },\n");
        html.push_str("            tooltip: {\n");
        html.push_str("                custom: function({series, seriesIndex, dataPointIndex, w}) {\n");
        html.push_str("                    var data = w.globals.initialSeries[seriesIndex].data[dataPointIndex];\n");
        html.push_str("                    return '<div class=\"arrow_box\">' +\n");
        html.push_str("                        '<span><b>Time:</b> ' + new Date(data.x).toLocaleString() + '</span><br>' +\n");
        html.push_str("                        '<span><b>Level:</b> ' + data.y + '</span><br>' +\n");
        html.push_str("                        '<span><b>Description:</b> ' + data.description + '</span>' +\n");
        html.push_str("                    '</div>';\n");
        html.push_str("                }\n");
        html.push_str("            }\n");
        html.push_str("        };\n");
        html.push_str("        var timelineChart = new ApexCharts(document.querySelector(\"#alertsTimeline\"), timelineOptions);\n");
        html.push_str("        timelineChart.render();\n");

        html.push_str("    </script>\n");
    }

    html.push_str("    </div>\n");
    html.push_str("</body>\n</html>");

    Ok(html)
}
