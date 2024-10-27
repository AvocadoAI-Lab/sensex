import EndpointCard from './EndpointCard';
import { AllResponses, WazuhResponse } from '../types/responses';

interface EndpointGroupProps {
  title: string;
  endpoints: {
    title: string;
    key: keyof AllResponses;
  }[];
  responses: AllResponses;
}

export default function EndpointGroup({ title, endpoints, responses }: EndpointGroupProps) {
  return (
    <div className="bg-gray-50 p-4 rounded-lg">
      <h2 className="text-lg font-semibold text-gray-900 mb-4">{title}</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {endpoints.map((endpoint) => (
          <EndpointCard
            key={endpoint.key}
            title={endpoint.title}
            response={responses[endpoint.key] as WazuhResponse | null}
          />
        ))}
      </div>
    </div>
  );
}
