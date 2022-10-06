declare const window: any;
export const allowedChainedSort = () =>{
    const data = window._env_.REACT_APP_ALLOWED_CHAINS

    let convertArray: any | undefined = data?.split(',')

    let allowedChain = [];

    for(let i = 0; i < convertArray?.length; i++){
        let name = convertArray[i].split(':')[0]
        let token = convertArray[i].split(':')[1]
        allowedChain.push({name: name, token: token})
    }
    return allowedChain;

}
