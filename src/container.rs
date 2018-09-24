#[derive(Serialize, Deserialize)]
struct Bridge {
    #[serde(rename = "IPAMConfig")]
    ipam_config: String,
    #[serde(rename = "Links")]
    links: String,
    #[serde(rename = "Aliases")]
    aliases: String,
    #[serde(rename = "NetworkID")]
    network_id: String,
    #[serde(rename = "EndpointID")]
    endpoint_id: String,
    #[serde(rename = "Gateway")]
    gateway: String,
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    ipv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    global_i_pv_6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_i_pv_6_prefix_len: i64,
    #[serde(rename = "MacAddress")]
    mac_address: String,
}

#[derive(Serialize, Deserialize)]
struct HostConfig {
    #[serde(rename = "NetworkMode")]
    network_mode: String,
}

#[derive(Serialize, Deserialize)]
struct Labels {
    #[serde(rename = "com.example.vendor")]
    com_example_vendor: String,
    #[serde(rename = "com.example.license")]
    com_example_license: String,
    #[serde(rename = "com.example.version")]
    com_example_version: String,
}

#[derive(Serialize, Deserialize)]
struct Mounts {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "Driver")]
    driver: String,
    #[serde(rename = "Mode")]
    mode: String,
    #[serde(rename = "RW")]
    rw: bool,
    #[serde(rename = "Propagation")]
    propagation: String,
}

#[derive(Serialize, Deserialize)]
struct NetworkSettings {
    #[serde(rename = "Networks")]
    networks: Networks,
}

#[derive(Serialize, Deserialize)]
struct Networks {
    bridge: Bridge,
}

#[derive(Serialize, Deserialize)]
struct Ports {
    #[serde(rename = "PrivatePort")]
    private_port: i64,
    #[serde(rename = "PublicPort")]
    public_port: i64,
    #[serde(rename = "Type")]
    _type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    #[serde(rename = "Id")]
   pub id: String,
    #[serde(rename = "Names")]
    names: Vec<String>,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "ImageID")]
    image_id: String,
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "Created")]
    created: i64,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Ports")]
    ports: Vec<Ports>,
    #[serde(rename = "Labels")]
    labels: Labels,
    #[serde(rename = "SizeRw")]
    size_rw: i64,
    #[serde(rename = "SizeRootFs")]
    size_root_fs: i64,
    #[serde(rename = "HostConfig")]
    host_config: HostConfig,
    #[serde(rename = "NetworkSettings")]
    network_settings: NetworkSettings,
    #[serde(rename = "Mounts")]
    mounts: Vec<Mounts>,
}
