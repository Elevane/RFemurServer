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

    void Start()
    {
        ConnectToServer();
    }

    void Update()
    {
        if (isConnected)
        {
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

        switch (operation)
        {
            case OperationType.MoveResponse:
                onEntityMove(content);
                break;

            default:
                Debug.LogWarning("Unknown operation received.");
                break;
        }
    }

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

    internal enum OperationType{
            ConnectServerRequest,
            ConnectServerRequestTokenResponse,
            ConnectGameRequest,
            MoveRequest,
            MoveResponse,
    }

}
