using System;
using System.IO;
using System.Net.Sockets;
using System.Text;
using UnityEngine;

public class TcpClientUnity : MonoBehaviour
{
    private TcpClient tcpClient;
    private NetworkStream networkStream;
    private StreamReader reader;
    private StreamWriter writer;

    
    private string serverAddress = "127.0.0.1";
    private int serverPort = 3333;

    private bool isConnected = false;

    // Start is called before the first frame update
    void Start()
    {
        ConnectToServer();
    }

    // Update is called once per frame
    void Update()
    {
        if (isConnected)
        {
            // Read message if available
            if (networkStream.DataAvailable)
            {
                string message = reader.ReadLine();
                if (!string.IsNullOrEmpty(message))
                {
                    ProcessMessage(message);
                }
            }
        }
    }

    // Connect to the server
    private void ConnectToServer()
    {
        try
        {
            tcpClient = new TcpClient(serverAddress, serverPort);
            networkStream = tcpClient.GetStream();
            reader = new StreamReader(networkStream, Encoding.UTF8);
            writer = new StreamWriter(networkStream, Encoding.UTF8);

            isConnected = true;
            Debug.Log("Connected to server.");
        }
        catch (Exception ex)
        {
            Debug.LogError("Error connecting to server: " + ex.Message);
        }
    }

    // Send a message to the server
    public void SendMessageToServer(string message)
    {
        if (isConnected)
        {
            try
            {
                writer.WriteLine(message);
                writer.Flush();
                Debug.Log("Sent message: " + message);
            }
            catch (Exception ex)
            {
                Debug.LogError("Error sending message: " + ex.Message);
            }
        }
    }

    // Process incoming messages
    private void ProcessMessage(string message)
    {
        string[] parts = message.Split('|');
        if (parts.Length != 3)
        {
            Debug.LogError("Invalid message format.");
            return;
        }

        string token = parts[0];
        string operation = parts[1];
        string content = parts[2];

        Debug.Log($"Received message: Token = {token}, Operation = {operation}, Content = {content}");

        // Perform different actions based on the operation
        switch (operation)
        {
            case "Login":
                // Handle login operation (this is just an example)
                HandleLogin(content);
                break;

            case "Update":
                // Handle data update operation
                HandleDataUpdate(content);
                break;

            default:
                Debug.LogWarning("Unknown operation received.");
                break;
        }
    }

    // Handle the login operation
    private void HandleLogin(string content)
    {
        // Parse the login content (this is an example, you can parse JSON or other formats here)
        Debug.Log("Handling login with content: " + content);
        // Example: Update player state or show UI
    }

    // Handle data update operation
    private void HandleDataUpdate(string content)
    {
        // Parse the content to update game state
        Debug.Log("Handling data update with content: " + content);
        // Example: Update game objects or variables based on the content
    }

    // Disconnect from the server
    private void OnApplicationQuit()
    {
        if (tcpClient != null && tcpClient.Connected)
        {
            writer.Close();
            reader.Close();
            networkStream.Close();
            tcpClient.Close();
        }

        Debug.Log("Disconnected from server.");
    }
}
