'use client';

interface CardProps {
    name: string,
    description: string,
    image: string,
    power: string,
    sharing_power: string,
    address: string
    index: number,
}

export const Card: React.FC<CardProps> = ({ 
    name, 
    description, 
    image,
    power,
    sharing_power,
    address,
    index
}) => {
    const handle = () => {
        window.open(`https://explorer.solana.com/address/${address}?cluster=devnet`, '_blank');
    };
    return (
        <div key={index} className="flex gap-x-4" > 
            <div className="card card-side bg-base-200 shadow-xl w-[600px] h-[350px] m-2">
                <figure className="w-[250px]"> 
                    <img
                        src={image}
                        alt="Movie" />
                </figure>
                <div className="card-body w-[350px] bg-slate-100">
                    <h2 className="card-title">{name}</h2>
                    <p>
                        {description}
                    </p>
                    <div className="my-4">
                        <p>Power: {power}</p>
                        <p>Sharing Power: {sharing_power}</p>
                    </div>
                    <div className="card-actions justify-end">
                        <button 
                        className="btn btn-primary"
                        onClick={() => handle()}>Watch</button>
                    </div>
                </div>
            </div>
        </div>
    )
}

